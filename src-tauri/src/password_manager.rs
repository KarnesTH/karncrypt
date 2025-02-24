use ring::rand::{SecureRandom, SystemRandom};

use crate::{Config, Database};

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
}
