use crate::utils::Encryption;
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
    pub fn new(master_key: Vec<u8>, user_id: i32, duration_minutes: u64) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            master_key,
            user_id,
            created_at: now,
            expires_at: now + (duration_minutes * 60),
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
    encryption: Encryption,
}

impl TokenManager {
    pub fn new(config_dir: PathBuf, encryption: Encryption) -> Self {
        Self {
            token_path: config_dir.join(".session_token"),
            encryption,
        }
    }

    pub fn create_session(
        &self,
        master_pass: &str,
        user_id: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!("Creating new session for user {}", user_id);
        let master_key = self.encryption.get_key(master_pass)?.into_bytes();

        let token = SessionToken::new(master_key, user_id, 30);

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

    pub fn has_valid_session(&self) -> bool {
        self.get_session().is_ok()
    }
}
