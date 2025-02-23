use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use chrono::Utc;

use super::database::{Database, User};

pub struct Auth {
    pub db: Database,
}

impl Auth {
    /// Create a new Auth instance.
    ///
    /// # Arguments
    ///
    /// * `db` - The database to use for authentication.
    ///
    /// # Returns
    ///
    /// A new Auth instance.
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    /// Register a new user.
    ///
    /// # Arguments
    ///
    /// * `username` - The username for the new user.
    /// * `master_pass` - The master password for the new user.
    ///
    /// # Returns
    ///
    /// A Result containing the new user or an error.
    ///
    /// # Errors
    ///
    /// If the user cannot be created.
    pub fn register(
        &self,
        username: &str,
        master_pass: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let encrypted_master = self.db.encryption.encrypt(master_pass).unwrap();
        let encode_master = STANDARD.encode(encrypted_master);

        let user = User {
            id: None,
            username: username.to_string(),
            master_key: encode_master.into_bytes(),
            created_at: Utc::now().to_rfc3339(),
            last_login: Utc::now().to_rfc3339(),
        };

        self.db.create(&user)?;

        Ok(())
    }

    /// Login a user.
    ///
    /// # Arguments
    ///
    /// * `username` - The username of the user.
    /// * `master_pass` - The master password of the user.
    ///
    /// # Returns
    ///
    /// A Result containing the user ID or an error.
    ///
    /// # Errors
    ///
    /// If the username or password is invalid.
    pub fn login(
        &self,
        username: &str,
        master_pass: &str,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let users = self.db.read_all::<User>()?;

        if let Some(user) = users.first() {
            if user.username != username {
                return Err("Invalid credentials".into());
            }

            let decode_master = STANDARD.decode(&user.master_key).unwrap();
            if let Ok(decrypted) = self.db.encryption.decrypt(&decode_master) {
                if decrypted == master_pass {
                    let mut updated_user = user.clone();
                    updated_user.last_login = Utc::now().to_rfc3339();
                    self.db.update(&updated_user)?;

                    return Ok(user.id.unwrap());
                }
            }
        }

        Err("Invalid credentials".into())
    }
}
