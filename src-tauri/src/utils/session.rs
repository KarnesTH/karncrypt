use crate::utils::Encryption;
use crate::Config;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
pub struct SessionToken {
    master_key: Vec<u8>,
    user_id: i32,
    created_at: u64,
    expires_at: u64,
}

impl SessionToken {
    /// Create a new session token
    ///
    /// # Arguments
    ///
    /// * `master_key` - The master key for the session
    /// * `user_id` - The user ID for the session
    /// * `duration_minutes` - The duration in minutes for the session
    ///
    /// # Returns
    ///
    /// A new session token
    pub fn new(master_key: Vec<u8>, user_id: i32, duration: u64) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            master_key,
            user_id,
            created_at: now,
            expires_at: now + duration,
        }
    }

    /// Check if the session token is valid
    ///
    /// # Returns
    ///
    /// True if the session token is valid, false otherwise
    pub fn is_valid(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        self.expires_at > now
    }

    /// Get the master key for the session
    ///
    /// # Returns
    ///
    /// The master key for the session
    pub fn get_master_key(&self) -> Vec<u8> {
        self.master_key.clone()
    }

    /// Get the user ID for the session
    ///
    /// # Returns
    ///
    /// The user ID for the session
    pub fn get_user_id(&self) -> i32 {
        self.user_id
    }
}

pub struct TokenManager {
    token_path: PathBuf,
    pub encryption: Encryption,
}

impl TokenManager {
    /// Create a new token manager
    ///
    /// # Arguments
    ///
    /// * `config_dir` - The directory to store the session token
    /// * `encryption` - The encryption instance to use
    ///
    /// # Returns
    ///
    /// A new token manager
    pub fn new(config_dir: PathBuf, encryption: Encryption) -> Self {
        Self {
            token_path: config_dir.join(".session_token"),
            encryption,
        }
    }

    /// Create a new session
    ///
    /// # Arguments
    ///
    /// * `master_pass` - The master password for the session
    /// * `user_id` - The user ID for the session
    ///
    /// # Returns
    ///
    /// A Result containing the session token or an error
    ///
    /// # Errors
    ///
    /// If the session token cannot be created
    pub fn create_session(
        &self,
        master_pass: &str,
        user_id: i32,
        duration: Option<u64>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!("Creating new session for user {}", user_id);
        let duration = match duration {
            Some(d) => d * 60,
            None => {
                let config = Config::load()?;
                config.app.auto_logout_duration * 60
            }
        };
        let master_key = self.encryption.get_key(master_pass)?.into_bytes();

        let token = SessionToken::new(master_key, user_id, duration);

        let token_str = serde_json::to_string(&token)?;
        let encrypted_token = self.encryption.encrypt(&token_str).unwrap();

        match fs::write(&self.token_path, encrypted_token) {
            Ok(_) => {
                info!("Successfully created session");
                Ok(())
            }
            Err(e) => {
                error!("Failed to write session token: {}", e);
                Err(e.into())
            }
        }
    }

    /// Get the session token
    ///
    /// # Returns
    ///
    /// A Result containing the session token or an error
    ///
    /// # Errors
    ///
    /// If the session token cannot be retrieved
    pub fn get_session(&self) -> Result<SessionToken, Box<dyn std::error::Error>> {
        if !self.token_path.exists() {
            info!("No session token found");
            return Err("No session token found".into());
        }

        let encrypted_token = fs::read(&self.token_path)?;
        let token_str = self.encryption.decrypt(&encrypted_token).unwrap();
        let token: SessionToken = serde_json::from_str(&token_str)?;

        if !token.is_valid() {
            info!("Session token expired");
            self.clear_session()?;
            return Err("Session token expired".into());
        }
        info!("Retrieved valid session token");

        Ok(token)
    }

    /// Clear the session token
    ///
    /// # Returns
    ///
    /// A Result containing the session token or an error
    ///
    /// # Errors
    ///
    /// If the session token cannot be cleared
    pub fn clear_session(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Clearing session");
        if self.token_path.exists() {
            match fs::remove_file(&self.token_path) {
                Ok(_) => info!("Successfully cleared session"),
                Err(e) => error!("Failed to clear session: {}", e),
            }
        }
        Ok(())
    }

    /// Check if a valid session exists
    ///
    /// # Returns
    ///
    /// True if a valid session exists, false otherwise
    pub fn has_valid_session(&self) -> bool {
        self.get_session().is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;
    use tempfile::TempDir;

    fn setup_test_manager() -> (TempDir, TokenManager) {
        let temp = TempDir::new().unwrap();
        let temp_path = temp.path().to_path_buf();
        let salt = [0u8; 16];
        let encryption = Encryption::new("test_password", &salt);
        (temp, TokenManager::new(temp_path, encryption))
    }

    #[test]
    fn test_session_token_creation() {
        let token = SessionToken::new(vec![1, 2, 3], 1, 30);

        assert_eq!(token.get_user_id(), 1);
        assert_eq!(token.get_master_key(), vec![1, 2, 3]);
        assert!(token.is_valid());
    }

    #[test]
    fn test_session_expiration() {
        let token = SessionToken::new(vec![1, 2, 3], 1, 1);
        assert!(token.is_valid());

        sleep(Duration::from_secs(2));
        assert!(!token.is_valid());
    }

    #[test]
    fn test_token_manager_workflow() {
        let (_temp, manager) = setup_test_manager();

        assert!(manager.create_session("test_pass", 1, Some(10)).is_ok());
        assert!(manager.has_valid_session());

        let session = manager.get_session().unwrap();
        assert_eq!(session.get_user_id(), 1);

        assert!(manager.clear_session().is_ok());
        assert!(!manager.has_valid_session());
    }

    #[test]
    fn test_invalid_session() {
        let (_temp, manager) = setup_test_manager();

        assert!(!manager.has_valid_session());
        assert!(manager.get_session().is_err());

        assert!(manager.create_session("test_pass", 1, Some(10)).is_ok());
        let token = SessionToken::new(vec![1, 2, 3], 1, 0);
        let token_str = serde_json::to_string(&token).unwrap();
        let encrypted = manager.encryption.encrypt(&token_str).unwrap();
        fs::write(&manager.token_path, encrypted).unwrap();

        assert!(!manager.has_valid_session());
        assert!(manager.get_session().is_err());
    }

    #[test]
    fn test_session_persistence() {
        let (_temp, manager) = setup_test_manager();

        manager.create_session("test_pass", 1, Some(10)).unwrap();

        assert!(manager.token_path.exists());

        manager.clear_session().unwrap();

        assert!(!manager.token_path.exists());
    }
}
