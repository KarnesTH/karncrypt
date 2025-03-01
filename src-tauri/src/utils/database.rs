use std::path::{Path, PathBuf};

use chrono::Utc;
use log::{error, info};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

use crate::utils::Encryption;

use super::models::Model;

#[derive(Serialize, Deserialize, Clone)]
pub struct PasswordEntry {
    pub id: Option<i32>,
    pub user_id: i32,
    pub service: String,
    pub username: String,
    pub password: String,
    pub url: String,
    pub notes: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub master_key: Vec<u8>,
    pub created_at: String,
    pub last_login: String,
}

pub struct Database {
    pub connection: Connection,
    pub path: PathBuf,
    pub encryption: Encryption,
}

impl Database {
    /// Create a new Database.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the database file.
    /// * `master_password` - The master password for the database.
    ///
    /// # Returns
    ///
    /// A Result containing the new Database or an error.
    ///
    /// # Errors
    ///
    /// If the database cannot be opened or the encryption key cannot be generated.
    pub fn new(
        path: PathBuf,
        master_password: &str,
        salt: &[u8; 16],
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let conn = Connection::open(&path)?;
        let encryption = Encryption::new(master_password, salt);
        let key = encryption.get_key(master_password)?;
        conn.execute_batch(&format!(
            "
                PRAGMA key = '{}';
                PRAGMA chiper_page_size = 4096;
                PRAGMA kdf_iter = 64000;
                PRAGMA chiper_memory_security = ON;
                PRAGMA foreign_keys = ON;
                PRAGMA journal_mode = WAL;
            ",
            key
        ))?;

        let db = Self {
            connection: conn,
            path,
            encryption,
        };

        db.create_tables()?;

        Ok(db)
    }

    /// Create the necessary tables in the database.
    ///
    /// This function creates the user and passwords tables in the database.
    /// If the user table already exists, a trigger is created to prevent more than one user from being created.
    ///
    /// # Returns
    ///
    /// A Result containing a unit or an error.
    ///
    /// # Errors
    ///
    /// If the table creation fails.
    fn create_tables(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.connection.execute_batch(
            "
                CREATE TABLE IF NOT EXISTS user (
                    id INTEGER PRIMARY KEY,
                    username TEXT NOT NULL UNIQUE,
                    master_key BLOB NOT NULL,
                    created_at TEXT NOT NULL,
                    last_login TEXT NOT NULL
                );

                CREATE TRIGGER IF NOT EXISTS single_user
                BEFORE INSERT ON user
                WHEN (SELECT COUNT(*) FROM user) >= 1
                BEGIN
                    SELECT RAISE(FAIL, 'Only one user allowed');
                END;

                CREATE TABLE IF NOT EXISTS passwords (
                    id INTEGER PRIMARY KEY,
                    user_id INTEGER NOT NULL,
                    service TEXT NOT NULL,
                    username TEXT NOT NULL,
                    password TEXT NOT NULL,
                    url TEXT NOT NULL,
                    notes TEXT NOT NULL,
                    created_at TEXT NOT NULL,
                    updated_at TEXT NOT NULL,
                    FOREIGN KEY (user_id) REFERENCES user(id) ON DELETE CASCADE
                );
            ",
        )?;
        Ok(())
    }

    /// Create a new database entry for a model.
    ///
    /// # Arguments
    ///
    /// * `model` - The model to create.
    ///
    /// # Returns
    ///
    /// A Result containing a unit or an error.
    ///
    /// # Errors
    ///
    /// If the insertion fails.
    pub fn create<T: Model>(&self, model: &T) -> Result<(), Box<dyn std::error::Error>> {
        info!("Creating new {} entry", T::table_name());
        let params = model.to_params();
        let fields: Vec<&str> = params.iter().map(|(name, _)| *name).collect();
        let placeholders: Vec<String> = (1..=params.len()).map(|i| format!("?{}", i)).collect();

        let query = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            T::table_name(),
            fields.join(", "),
            placeholders.join(", ")
        );

        let values: Vec<&dyn rusqlite::ToSql> = params.iter().map(|(_, value)| *value).collect();

        match self.connection.execute(&query, &values[..]) {
            Ok(_) => {
                info!("Successfully created {} entry", T::table_name());
                Ok(())
            }
            Err(e) => {
                error!("Failed to create {} entry: {}", T::table_name(), e);
                Err(e.into())
            }
        }
    }

    /// Read all entries for a model.
    ///
    /// # Returns
    ///
    /// A Result containing a vector of model objects or an error.
    ///
    /// # Errors
    ///
    /// If the query fails.
    pub fn read_all<T: Model>(&self) -> Result<Vec<T>, Box<dyn std::error::Error>> {
        info!("Reading all entries from {}", T::table_name());
        let mut stmt = self
            .connection
            .prepare(&format!("SELECT * FROM {}", T::table_name()))?;
        let entries = stmt.query_map([], |row| T::from_row(row))?;

        let mut result = Vec::new();
        for entry in entries {
            result.push(entry?);
        }

        match result.len() {
            0 => info!("No entries found in {}", T::table_name()),
            n => info!("Found {} entries in {}", n, T::table_name()),
        }

        Ok(result)
    }

    /// Read a model entry by ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the entry to read.
    ///
    /// # Returns
    ///
    /// A Result containing the model object or an error.
    ///
    /// # Errors
    ///
    /// If the query fails.
    pub fn read_by_id<T: Model>(&self, id: i32) -> Result<T, Box<dyn std::error::Error>> {
        info!("Reading {} entry with id {}", T::table_name(), id);
        let mut stmt = self
            .connection
            .prepare(&format!("SELECT * FROM {} WHERE id = ?1", T::table_name()))?;
        let result = stmt.query_row([id], |row| T::from_row(row))?;

        Ok(result)
    }

    /// Update a model entry.
    ///
    /// # Arguments
    ///
    /// * `model` - The model to update.
    ///
    /// # Returns
    ///
    /// A Result containing a unit or an error.
    ///
    /// # Errors
    ///
    /// If the update fails.
    pub fn update<T: Model>(&self, model: &T) -> Result<(), Box<dyn std::error::Error>> {
        info!("Updating {} entry", T::table_name());
        let params = model.to_params();
        let fields: Vec<String> = params
            .iter()
            .enumerate()
            .map(|(i, (name, _))| format!("{} = ?{}", name, i + 1))
            .collect();

        let query = format!(
            "UPDATE {} SET {} WHERE id = ?{}",
            T::table_name(),
            fields.join(", "),
            params.len() + 1
        );

        let id = model.get_id().ok_or("Model does not have an ID")?;

        let mut param_values: Vec<&dyn rusqlite::ToSql> =
            params.iter().map(|(_, value)| *value).collect();
        param_values.push(&id);

        match self.connection.execute(&query, &param_values[..]) {
            Ok(_) => {
                info!("Successfully updated {} entry", T::table_name());
                Ok(())
            }
            Err(e) => {
                error!("Failed to update {} entry: {}", T::table_name(), e);
                Err(e.into())
            }
        }
    }

    /// Delete a model entry by ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the entry to delete.
    ///
    /// # Returns
    ///
    /// A Result containing a unit or an error.
    pub fn delete<T: Model>(&self, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        info!("Deleting {} entry with id {}", T::table_name(), id);

        match self.connection.execute(
            &format!("DELETE FROM {} WHERE id = ?1", T::table_name()),
            [id],
        ) {
            Ok(_) => {
                info!("Successfully deleted {} entry", T::table_name());
                Ok(())
            }
            Err(e) => {
                error!("Failed to delete {} entry: {}", T::table_name(), e);
                Err(e.into())
            }
        }
    }

    /// Create a database dump.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to save the dump file.
    /// * `master_password` - The master password for the database.
    ///
    /// # Returns
    ///
    /// A Result containing a unit or an error.
    ///
    /// # Errors
    ///
    /// If the dump cannot be created.
    pub fn create_dump(
        &self,
        path: &Path,
        master_password: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let key = self.encryption.get_key(master_password)?;

        self.connection.execute_batch(&format!(
            "ATTACH DATABASE '{}' AS backup KEY '{}';",
            path.to_str().ok_or("Invalid path")?,
            key
        ))?;

        self.connection
            .execute_batch("SELECT sqlcipher_export('backup');")?;

        self.connection.execute_batch("DETACH DATABASE backup;")?;

        let verify_conn = Connection::open(path)?;
        let result = verify_conn.execute_batch("SELECT count(*) FROM sqlite_master;");

        if result.is_ok() {
            return Err("Database dump is not encrypted!".into());
        }

        Ok(())
    }

    /// Restore a database from a dump file.
    ///
    /// # Arguments
    ///
    /// * `dump_path` - The path to the dump file.
    /// * `master_password` - The master password for the database.
    ///
    /// # Returns
    ///
    /// A Result containing a unit or an error.
    ///
    /// # Errors
    ///
    /// If the dump cannot be restored.
    pub fn restore_from_dump(
        &self,
        dump_path: &Path,
        master_password: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let key = self.encryption.get_key(master_password)?;

        self.connection
            .execute_batch(&format!("PRAGMA key = '{}'", key))?;

        self.connection.execute_batch(&format!(
            "ATTACH DATABASE '{}' AS dump KEY '{}'",
            dump_path.to_str().ok_or("Invalid path")?,
            key
        ))?;

        self.connection.execute_batch(
            "BEGIN TRANSACTION;
                 DELETE FROM passwords;
                 DELETE FROM user;
                 INSERT INTO user SELECT * FROM dump.user;
                 INSERT INTO passwords SELECT * FROM dump.passwords;
                 COMMIT;",
        )?;

        self.connection.execute_batch("DETACH DATABASE dump")?;

        Ok(())
    }
}

impl PasswordEntry {
    /// Create a new PasswordEntry.
    ///
    /// # Arguments
    ///
    /// * `service` - The name of the service.
    /// * `username` - The username for the service.
    /// * `password` - The password for the service.
    /// * `url` - The URL for the service.
    /// * `notes` - Any notes for the service.
    ///
    /// # Returns
    ///
    /// A new PasswordEntry.
    pub fn new(
        user_id: i32,
        service: String,
        username: String,
        password: String,
        url: String,
        notes: String,
    ) -> Self {
        Self {
            id: None,
            user_id,
            service,
            username,
            password,
            url,
            notes,
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Auth;

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
    fn test_database_creation() {
        let (_temp, db) = setup_test_db();

        let tables = db
            .connection
            .prepare("SELECT name FROM sqlite_master WHERE type='table'")
            .unwrap()
            .query_map([], |row| row.get::<_, String>(0))
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert!(tables.contains(&"user".to_string()));
        assert!(tables.contains(&"passwords".to_string()));
    }

    #[test]
    fn test_single_user_constraint() {
        let (_temp, db) = setup_test_db();

        let user1 = User {
            id: None,
            username: "test1".to_string(),
            master_key: vec![1, 2, 3],
            created_at: Utc::now().to_rfc3339(),
            last_login: Utc::now().to_rfc3339(),
        };
        assert!(db.create(&user1).is_ok());

        let user2 = User {
            id: None,
            username: "test2".to_string(),
            master_key: vec![4, 5, 6],
            created_at: Utc::now().to_rfc3339(),
            last_login: Utc::now().to_rfc3339(),
        };
        assert!(db.create(&user2).is_err());
    }

    #[test]
    fn test_password_entry_crud() {
        let (_temp, db) = setup_test_db();

        let user = User {
            id: None,
            username: "test".to_string(),
            master_key: vec![1, 2, 3],
            created_at: Utc::now().to_rfc3339(),
            last_login: Utc::now().to_rfc3339(),
        };
        db.create(&user).unwrap();

        let entry = PasswordEntry::new(
            1,
            "service".to_string(),
            "username".to_string(),
            "password".to_string(),
            "url".to_string(),
            "notes".to_string(),
        );

        assert!(db.create(&entry).is_ok());

        let entries = db.read_all::<PasswordEntry>().unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].service, "service");

        let read_entry = db.read_by_id::<PasswordEntry>(1).unwrap();
        assert_eq!(read_entry.service, "service");

        let mut updated_entry = entries[0].clone();
        updated_entry.service = "new_service".to_string();
        assert!(db.update(&updated_entry).is_ok());

        let updated = db.read_by_id::<PasswordEntry>(1).unwrap();
        assert_eq!(updated.service, "new_service");

        assert!(db.delete::<PasswordEntry>(1).is_ok());

        let entries = db.read_all::<PasswordEntry>().unwrap();
        assert!(entries.is_empty());
    }

    #[test]
    fn test_foreign_key_constraint() {
        let (_temp, db) = setup_test_db();

        let entry = PasswordEntry::new(
            999,
            "service".to_string(),
            "username".to_string(),
            "password".to_string(),
            "url".to_string(),
            "notes".to_string(),
        );

        assert!(db.create(&entry).is_err());
    }

    #[test]
    fn test_user_cascade_delete() {
        let (_temp, db) = setup_test_db();

        let user = User {
            id: None,
            username: "test".to_string(),
            master_key: vec![1, 2, 3],
            created_at: Utc::now().to_rfc3339(),
            last_login: Utc::now().to_rfc3339(),
        };
        db.create(&user).unwrap();

        let entry = PasswordEntry::new(
            1,
            "service".to_string(),
            "username".to_string(),
            "password".to_string(),
            "url".to_string(),
            "notes".to_string(),
        );
        db.create(&entry).unwrap();

        db.delete::<User>(1).unwrap();

        let entries = db.read_all::<PasswordEntry>().unwrap();
        assert!(entries.is_empty());
    }

    #[test]
    fn test_database_dump() {
        let (temp, db) = setup_test_db();

        let auth = Auth::new(&db);
        auth.register("testuser", "testpass").unwrap();

        let dump_path = temp.path().join("test_dump.db");
        db.create_dump(&dump_path, "test_password").unwrap();

        assert!(dump_path.exists());

        let unencrypted_access = Connection::open(&dump_path);
        assert!(unencrypted_access.is_ok());
        assert!(unencrypted_access
            .unwrap()
            .execute_batch("SELECT count(*) FROM sqlite_master;")
            .is_err());
    }

    #[test]
    fn test_database_restore() {
        let (temp, db) = setup_test_db();

        let auth = Auth::new(&db);
        auth.register("testuser", "testpass").unwrap();

        let dump_path = temp.path().join("test_dump.db");
        db.create_dump(&dump_path, "test_password").unwrap();

        let new_db_path = temp.path().join("new.db");
        let new_db = Database::new(new_db_path, "test_password", &[0u8; 16]).unwrap();

        new_db
            .restore_from_dump(&dump_path, "test_password")
            .unwrap();

        let users = new_db.read_all::<User>().unwrap();
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].username, "testuser");
    }
}
