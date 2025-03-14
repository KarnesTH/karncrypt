use std::collections::HashMap;

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use chrono::Utc;
use ring::rand::{SecureRandom, SystemRandom};

use crate::{
    utils::{PasswordHealth, User},
    Auth, Config, Database, Encryption, PasswordEntry, TokenManager,
};

pub struct PasswordManager {
    pub db: Database,
    token_manager: TokenManager,
}

impl PasswordManager {
    /// Create a new PasswordManager instance.
    ///
    /// # Arguments
    ///
    /// * `master_pass` - The master password to use for the database.
    ///
    /// # Returns
    ///
    /// A Result containing the PasswordManager instance or an error.
    ///
    /// # Errors
    ///
    /// If the database cannot be created.
    pub fn new(master_pass: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config = Config::load()?;
        let config_dir = Config::get_config_dir()?;
        let salt_file = config_dir.join(".salt");
        let db_dir = config.get_db_dir()?;
        let db_path = db_dir.join(config.database.db_name.as_str());

        let salt = if salt_file.exists() {
            let salt_data = std::fs::read(&salt_file)?;
            salt_data[..16].try_into()?
        } else {
            let mut new_salt = [0u8; 16];
            SystemRandom::new().fill(&mut new_salt).unwrap();
            std::fs::write(&salt_file, new_salt)?;
            new_salt
        };

        let encryption = Encryption::new(master_pass, &salt);
        let db = Database::new(db_path, master_pass, &salt)?;
        let token_manager = TokenManager::new(config_dir, encryption);

        Ok(Self { db, token_manager })
    }

    /// Login to an new session.
    ///
    /// # Arguments
    ///
    /// * `username` - The username to login.
    /// * `master_pass` - The master password to use for the database.
    ///
    /// # Returns
    ///
    /// A Result containing the user's ID or an error.
    ///
    /// # Errors
    ///
    /// If the user cannot be logged in.
    pub fn login(
        &mut self,
        username: &str,
        master_pass: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.token_manager.has_valid_session() {
            self.token_manager.clear_session()?;
        }

        let auth = Auth::new(&self.db);
        let user_id = auth.login(username, master_pass)?;

        self.token_manager
            .create_session(master_pass, user_id, None)?;

        Ok(())
    }

    /// Logout of the current session.
    ///
    /// # Returns
    ///
    /// A Result containing a unit or an error.
    ///
    /// # Errors
    ///
    /// If the session cannot be cleared.
    pub fn logout(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.token_manager.clear_session()
    }

    /// Check if the user is logged in.
    ///
    /// # Returns
    ///
    /// A boolean indicating if the user is logged in.
    pub fn is_logged_in(&self) -> bool {
        self.token_manager.has_valid_session()
    }

    /// Cleanup any session tokens on startup.
    ///
    /// # Returns
    ///
    /// A Result containing a unit or an error.
    ///
    /// # Errors
    ///
    /// If the session token cannot be removed.
    pub fn cleanup_on_startup() -> Result<(), Box<dyn std::error::Error>> {
        let config_dir = Config::get_config_dir()?;
        let session_path = config_dir.join(".session_token");

        if session_path.exists() {
            std::fs::remove_file(&session_path)?;
        }

        Ok(())
    }

    /// Cleanup any session tokens on exit.
    ///
    /// # Returns
    ///
    /// A Result containing a unit or an error.
    ///
    /// # Errors
    ///
    /// If the session token cannot be removed.
    pub fn cleanup_on_exit(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.token_manager.clear_session()
    }

    /// Add a new password entry to the database.
    ///
    /// # Arguments
    ///
    /// * `service` - The name of the service the password is for.
    /// * `username` - The username for the service.
    /// * `password` - The password for the service.
    /// * `url` - The URL for the service.
    /// * `notes` - Optional notes for the password.
    ///
    /// # Returns
    ///
    /// A Result containing the new password entry or an error.
    ///
    /// # Errors
    ///
    /// If the notes are too long.
    /// If the password entry cannot be added to the database.
    pub fn add_password(
        &self,
        service: String,
        username: String,
        password: String,
        url: String,
        notes: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let session = self.token_manager.get_session()?;
        let user_id = session.get_user_id();

        let encryptd = self.db.encryption.encrypt(&password).unwrap();
        let encoded = STANDARD.encode(&encryptd);

        let notes = if let Some(notes) = notes {
            if notes.len() > 1000 {
                return Err("Notes must be less than 1000 characters".into());
            }

            notes
        } else {
            "".to_string()
        };

        let model = PasswordEntry {
            id: None,
            user_id,
            service,
            username,
            password: encoded,
            url,
            notes,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };

        self.db.create(&model)?;

        Ok(())
    }

    /// Update a password entry in the database.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the password entry to update.
    /// * `service` - The name of the service the password is for.
    /// * `username` - The username for the service.
    /// * `password` - The password for the service.
    /// * `url` - The URL for the service.
    /// * `notes` - Optional notes for the password.
    ///
    /// # Returns
    ///
    /// A Result containing the updated password entry or an error.
    ///
    /// # Errors
    ///
    /// If the notes are too long.
    /// If the password entry cannot be updated.
    pub fn update_password(
        &self,
        id: i32,
        service: String,
        username: String,
        password: String,
        url: String,
        notes: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let session = self.token_manager.get_session()?;
        let user_id = session.get_user_id();

        let encryptd = self.db.encryption.encrypt(&password).unwrap();
        let encoded = STANDARD.encode(&encryptd);

        let notes = if let Some(notes) = notes {
            if notes.len() > 1000 {
                return Err("Notes must be less than 1000 characters".into());
            }

            notes
        } else {
            "".to_string()
        };

        let model = PasswordEntry {
            id: Some(id),
            user_id,
            service,
            username,
            password: encoded,
            url,
            notes,
            created_at: "".to_string(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };

        self.db.update(&model)?;

        Ok(())
    }

    /// Delete a password entry from the database.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the password entry to delete.
    ///
    /// # Returns
    ///
    /// A Result containing a unit or an error.
    ///
    /// # Errors
    ///
    /// If the password entry cannot be deleted.
    pub fn delete_password(&self, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        let session = self.token_manager.get_session()?;

        let password = self.db.read_by_id::<PasswordEntry>(id)?;

        if password.user_id != session.get_user_id() {
            return Err("Unauthorized".into());
        }

        self.db.delete::<PasswordEntry>(id)?;

        Ok(())
    }

    /// Get all password entries from the database.
    ///
    /// # Returns
    ///
    /// A Result containing a vector of password entries or an error.
    ///
    /// # Errors
    ///
    /// If the password entries cannot be retrieved.
    pub fn get_passwords(&self) -> Result<Vec<PasswordEntry>, Box<dyn std::error::Error>> {
        let session = self.token_manager.get_session()?;
        let user_id = session.get_user_id();

        let passwords = self.db.read_all::<PasswordEntry>()?;
        Ok(passwords
            .into_iter()
            .filter(|p| p.user_id == user_id)
            .collect())
    }

    /// Updates the Users master password.
    ///
    /// # Arguments
    ///
    /// * `current_password` - The current master password.
    /// * `new_password` - The new master password.
    ///
    /// # Returns
    ///
    /// A Result containing a unit or an error.
    ///
    /// # Errors
    ///
    /// If the current password is incorrect.
    pub fn update_master_password(
        &mut self,
        current_password: &str,
        new_password: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let session = self.token_manager.get_session()?;
        let user_id = session.get_user_id();
        let current_user = self.db.read_by_id::<User>(user_id)?;

        let current_key = self.db.encryption.get_key(current_password)?;
        let new_key = self.db.encryption.get_key(new_password)?;

        self.db.connection.execute_batch(&format!(
            "PRAGMA key = '{}';
            PRAGMA rekey = '{}';",
            current_key, new_key
        ))?;

        let config_dir = Config::get_config_dir()?;
        let salt_file = config_dir.join(".salt");
        let salt: [u8; 16] = std::fs::read(&salt_file)?[..16].try_into()?;

        self.db.encryption = Encryption::new(new_password, &salt);
        self.token_manager.encryption = self.db.encryption.clone();

        let encryptd = self.db.encryption.encrypt(new_password).unwrap();
        let encoded = STANDARD.encode(&encryptd);

        let model = User {
            id: Some(user_id),
            username: current_user.username,
            master_key: encoded.into_bytes(),
            created_at: current_user.created_at,
            last_login: current_user.last_login,
        };

        self.db.update(&model)?;

        self.token_manager.clear_session()?;

        Ok(())
    }

    /// Generate a new password.
    ///
    /// # Arguments
    ///
    /// * `length` - The length of the password to generate.
    ///
    /// # Returns
    ///
    /// A Result containing the generated password or an error.
    ///
    /// # Errors
    ///
    /// If the password cannot be generated.
    pub fn generate_password(length: usize) -> Result<String, Box<dyn std::error::Error>> {
        if !Self::is_valid_password_length(&length.to_string()) {
            return Err("Invalid password length".into());
        }

        let charset: &[u8] =
            b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()-_=+";
        let rng = SystemRandom::new();
        let mut password = String::with_capacity(length);

        for _ in 0..length {
            let mut byte = [0u8; 1];
            rng.fill(&mut byte).unwrap();
            let idx = byte[0] as usize % charset.len();
            password.push(charset[idx] as char);
        }

        if Self::is_valid_password(&password) {
            Ok(password)
        } else {
            Self::generate_password(length)
        }
    }

    /// Check if a password is valid.
    ///
    /// A valid password must contain at least one lowercase letter, one uppercase letter, one digit, and one special character.
    ///
    /// # Arguments
    ///
    /// * `password` - The password to check.
    ///
    /// # Returns
    ///
    /// A boolean indicating if the password is valid.
    fn is_valid_password(password: &str) -> bool {
        let mut has_lower = false;
        let mut has_upper = false;
        let mut has_digit = false;
        let mut has_special = false;

        for c in password.chars() {
            if c.is_lowercase() {
                has_lower = true;
            } else if c.is_uppercase() {
                has_upper = true;
            } else if c.is_ascii_digit() {
                has_digit = true;
            } else {
                has_special = true;
            }
        }

        has_lower && has_upper && has_digit && has_special
    }

    /// Check if a password length is valid.
    ///
    /// A valid password length must be between 8 and 64 characters.
    ///
    /// # Arguments
    ///
    /// * `length` - The length of the password.
    ///
    /// # Returns
    ///
    /// A boolean indicating if the password length is valid.
    fn is_valid_password_length(length: &str) -> bool {
        if let Ok(length) = length.parse::<usize>() {
            (8..=64).contains(&length)
        } else {
            false
        }
    }

    /// Verify the master password.
    ///
    /// # Arguments
    ///
    /// * `master_pass` - The master password to verify
    ///
    /// # Returns
    ///
    /// A Result containing a boolean indicating if the password is correct
    ///
    /// # Errors
    ///
    /// If the verification fails
    pub fn verify_master_password(
        &self,
        master_pass: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let session = self.token_manager.get_session()?;
        let user_id = session.get_user_id();
        let user = self.db.read_by_id::<User>(user_id)?;

        let decoded = STANDARD.decode(&user.master_key)?;
        let decrypted = self.db.encryption.decrypt(&decoded).unwrap();

        Ok(decrypted == master_pass)
    }

    /// Decrypt a password.
    ///
    /// # Arguments
    ///
    /// * `password` - The password to decrypt.
    ///
    /// # Returns
    ///
    /// A Result containing the decrypted password or an error.
    ///
    /// # Errors
    ///
    /// If the password cannot be decrypted.
    pub fn decrypt_password(&self, password: &str) -> Result<String, Box<dyn std::error::Error>> {
        let decoded = STANDARD.decode(password.as_bytes()).unwrap();
        let decrypted = self.db.encryption.decrypt(&decoded).unwrap();
        Ok(decrypted)
    }

    /// Check the health of a password.
    ///
    /// # Arguments
    ///
    /// * `password` - The password to check.
    ///
    /// # Returns
    ///
    /// A Result containing the password health or an error.
    ///
    /// # Errors
    ///
    /// If the password health cannot be checked.
    pub fn check_password_health(
        &self,
        service: &str,
        username: &str,
        password: &str,
    ) -> Result<PasswordHealth, Box<dyn std::error::Error>> {
        let mut health = PasswordHealth::new(
            service.to_string(),
            username.to_string(),
            password,
            Utc::now(),
        );
        health.analyze()?;

        Ok(health)
    }

    /// Check the health of all passwords.
    ///
    /// # Returns
    ///
    /// A Result containing a vector of password healths or an error.
    ///
    /// # Errors
    ///
    /// If the password healths cannot be checked.
    pub fn check_passwords_health(
        &self,
    ) -> Result<Vec<PasswordHealth>, Box<dyn std::error::Error>> {
        let passwords = self.get_passwords()?;
        let mut healths = Vec::new();
        let mut raw_passwords = Vec::new();

        for password in &passwords {
            let decoded = STANDARD.decode(password.password.as_bytes()).unwrap();
            let decrypted = self.db.encryption.decrypt(&decoded).unwrap();
            raw_passwords.push(decrypted.clone());
        }

        let mut password_counts = HashMap::new();
        for pw in &raw_passwords {
            *password_counts.entry(pw.clone()).or_insert(0) += 1;
        }

        for (_, password) in passwords.iter().enumerate() {
            let decoded = STANDARD.decode(password.password.as_bytes()).unwrap();
            let decrypted = self.db.encryption.decrypt(&decoded).unwrap();

            let mut health = PasswordHealth::new(
                password.service.clone(),
                password.username.clone(),
                &decrypted,
                password.updated_at.parse()?,
            );

            health.analyze()?;

            if password_counts.get(&decrypted).unwrap_or(&0) > &1 {
                health.set_duplicate(true);
            }

            healths.push(health);
        }

        Ok(healths)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_generation() {
        let password = PasswordManager::generate_password(16).unwrap();
        assert_eq!(password.len(), 16);
        assert!(PasswordManager::is_valid_password(&password));

        assert!(PasswordManager::generate_password(7).is_err());
        assert!(PasswordManager::generate_password(65).is_err());
    }

    #[test]
    fn test_password_validation() {
        assert!(PasswordManager::is_valid_password("Test123!@#"));

        assert!(!PasswordManager::is_valid_password("nouppercasetest123!"));
        assert!(!PasswordManager::is_valid_password("NOLOWERCASETEST123!"));
        assert!(!PasswordManager::is_valid_password("NoSpecialChars123"));
        assert!(!PasswordManager::is_valid_password("No-Numbers-Here!"));
    }
}
