use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use chrono::Utc;
use log::{error, info};

use super::database::{Database, User};

pub struct Auth<'a> {
    pub db: &'a Database,
}

impl<'a> Auth<'a> {
    /// Create a new Auth instance.
    ///
    /// # Arguments
    ///
    /// * `db` - The database to use for authentication.
    ///
    /// # Returns
    ///
    /// A new Auth instance.
    pub fn new(db: &'a Database) -> Self {
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
        info!("Attempting to register new user: {}", username);

        let encrypted_master = self.db.encryption.encrypt(master_pass).unwrap();
        let encode_master = STANDARD.encode(encrypted_master);

        let user = User {
            id: None,
            username: username.to_string(),
            master_key: encode_master.into_bytes(),
            created_at: Utc::now().to_rfc3339(),
            last_login: Utc::now().to_rfc3339(),
        };

        match self.db.create(&user) {
            Ok(_) => {
                info!("Successfully registered new user: {}", username);
                Ok(())
            }
            Err(e) => {
                error!("Failed to register user: {}", e);
                Err(e)
            }
        }
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
        info!("Attempting to verify credentials");

        let users = self.db.read_all::<User>()?;
        info!("Found {} users", users.len());

        if let Some(user) = users.first() {
            if user.username != username {
                error!("Username mismatch");
                return Err("Invalid credentials".into());
            }

            let decode_master = STANDARD.decode(&user.master_key).unwrap();
            if let Ok(decrypted) = self.db.encryption.decrypt(&decode_master) {
                if decrypted == master_pass {
                    info!("Password verified successfully");
                    let mut updated_user = user.clone();
                    updated_user.last_login = Utc::now().to_rfc3339();
                    self.db.update(&updated_user)?;

                    return Ok(user.id.unwrap());
                }
            }
            error!("Password verification failed");
        } else {
            error!("No users found in database");
        }

        Err("Invalid credentials".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn setup_test_db() -> (TempDir, Database) {
        let temp = TempDir::new().unwrap();
        let db_path = temp.path().join("test.db");
        let salt = [0u8; 16];
        let db = Database::new(db_path, "test_password", &salt).unwrap();
        (temp, db)
    }

    #[test]
    fn test_registration() {
        let (_temp, db) = setup_test_db();
        let auth = Auth::new(&db);

        let result = auth.register("testuser", "testpass");
        assert!(result.is_ok());

        let result = auth.register("testuser2", "testpass");
        assert!(result.is_err());
    }

    #[test]
    fn test_login() {
        let (_temp, db) = setup_test_db();
        let auth = Auth::new(&db);

        auth.register("testuser", "testpass").unwrap();

        let result = auth.login("testuser", "testpass");
        assert!(result.is_ok());

        let result = auth.login("testuser", "wrongpass");
        assert!(result.is_err());

        let result = auth.login("wronguser", "testpass");
        assert!(result.is_err());
    }

    #[test]
    fn test_last_login_update() {
        let (_temp, db) = setup_test_db();
        let auth = Auth::new(&db);

        auth.register("testuser", "testpass").unwrap();

        let users = db.read_all::<User>().unwrap();
        let first_login = users[0].last_login.clone();

        std::thread::sleep(std::time::Duration::from_secs(1));
        auth.login("testuser", "testpass").unwrap();

        let users = db.read_all::<User>().unwrap();
        assert_ne!(first_login, users[0].last_login);
    }

    #[test]
    fn test_password_encryption() {
        let (_temp, db) = setup_test_db();
        let auth = Auth::new(&db);

        auth.register("testuser", "testpass").unwrap();

        let users = db.read_all::<User>().unwrap();
        let stored_key = &users[0].master_key;

        assert_ne!(stored_key, "testpass".as_bytes());
    }
}
