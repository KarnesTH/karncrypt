use ring::rand::{SecureRandom, SystemRandom};

use crate::{Config, Database, PasswordEntry};

pub struct PasswordManager {
    pub db: Database,
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

        let salt = if salt_file.exists() {
            let salt_data = std::fs::read(&salt_file)?;
            salt_data[..16].try_into()?
        } else {
            let mut new_salt = [0u8; 16];
            SystemRandom::new().fill(&mut new_salt).unwrap();
            std::fs::write(&salt_file, &new_salt)?;
            new_salt
        };

        let db = Database::new(config.get_db_path()?, master_pass, &salt)?;

        Ok(Self { db })
    }

    /// Add a new password entry to the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user to add the password for.
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
        user_id: i32,
        service: String,
        username: String,
        password: String,
        url: String,
        notes: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
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
            password,
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
            user_id: 0,
            service,
            username,
            password,
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
        self.db.read_all()
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
}
