use std::path::PathBuf;

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use chrono::Utc;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

use crate::utils::Encryption;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PasswordEntry {
    pub id: Option<i32>,
    pub service: String,
    pub username: String,
    pub password: String,
    pub url: String,
    pub notes: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug)]
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

        conn.execute(
            "CREATE TABLE IF NOT EXISTS passwords (
                id INTEGER PRIMARY KEY,
                service TEXT NOT NULL,
                username TEXT NOT NULL,
                password TEXT NOT NULL,
                url TEXT NOT NULL,
                notes TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;

        Ok(Self {
            connection: conn,
            path,
            encryption,
        })
    }

    /// Create a new password entry.
    ///
    /// # Arguments
    ///
    /// * `entry` - The password entry to create.
    ///
    /// # Returns
    ///
    /// A Result containing a unit or an error.
    ///
    /// # Errors
    ///
    /// If the insertion fails.
    pub fn create(&self, entry: &PasswordEntry) -> Result<(), Box<dyn std::error::Error>> {
        let encrypted_pass = self.encryption.encrypt(&entry.password).unwrap();
        let encoded_pass = STANDARD.encode(&encrypted_pass);

        self.connection.execute(
            "INSERT INTO passwords (service, username, password, url, notes, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                entry.service,
                entry.username,
                encoded_pass,
                entry.url,
                entry.notes,
                Utc::now().to_rfc3339(),
                Utc::now().to_rfc3339(),
            ],
        )?;

        Ok(())
    }

    /// Read all password entries.
    ///
    /// # Returns
    ///
    /// A Result containing a vector of PasswordEntry objects or an error.
    ///
    /// # Errors
    ///
    /// If the query fails.
    pub fn read_all(&self) -> Result<Vec<PasswordEntry>, Box<dyn std::error::Error>> {
        let mut stmt = self.connection.prepare("SELECT * FROM passwords")?;
        let entries = stmt.query_map([], |row| {
            let encoded_pass: String = row.get(3)?;
            let decoded_pass = STANDARD.decode(encoded_pass).unwrap();
            let pass = self.encryption.decrypt(&decoded_pass).unwrap();

            Ok(PasswordEntry {
                id: row.get(0)?,
                service: row.get(1)?,
                username: row.get(2)?,
                password: pass,
                url: row.get(4)?,
                notes: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })?;

        let mut result = Vec::new();
        for entry in entries {
            result.push(entry?);
        }

        Ok(result)
    }

    /// Read a password entry by ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the password entry.
    ///
    /// # Returns
    ///
    /// A Result containing a PasswordEntry object or an error.
    ///
    /// # Errors
    ///
    /// If the query fails.
    pub fn read_by_id(&self, id: i32) -> Result<PasswordEntry, Box<dyn std::error::Error>> {
        let mut stmt = self
            .connection
            .prepare("SELECT * FROM passwords WHERE id = ?1")?;
        let entry = stmt.query_map(params![id], |row| {
            let encoded_pass: String = row.get(3)?;
            let decoded_pass = STANDARD.decode(encoded_pass).unwrap();
            let pass = self.encryption.decrypt(&decoded_pass).unwrap();

            Ok(PasswordEntry {
                id: row.get(0)?,
                service: row.get(1)?,
                username: row.get(2)?,
                password: pass,
                url: row.get(4)?,
                notes: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })?;

        let mut result = Vec::new();
        for entry in entry {
            result.push(entry?);
        }

        Ok(result[0].clone())
    }

    /// Update a password entry by ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the password entry.
    ///
    /// # Returns
    ///
    /// A Result containing a unit or an error.
    ///
    /// # Errors
    ///
    /// If the update fails.
    pub fn update(&self, id: i32, entry: PasswordEntry) -> Result<(), Box<dyn std::error::Error>> {
        let encrypted_pass = self.encryption.encrypt(&entry.password).unwrap();
        let encoded_pass = STANDARD.encode(&encrypted_pass);

        self.connection.execute(
            "UPDATE passwords SET service = ?1, username = ?2, password = ?3, url = ?4, notes = ?5, updated_at = ?6 WHERE id = ?7",
            params![
                entry.service,
                entry.username,
                encoded_pass,
                entry.url,
                entry.notes,
                Utc::now().to_rfc3339(),
                id,
            ],
        )?;

        Ok(())
    }

    /// Delete a password entry by ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the password entry.
    ///
    /// # Returns
    ///
    /// A Result containing a unit or an error.
    ///
    /// # Errors
    ///
    /// If the deletion fails.
    pub fn delete(&self, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        self.connection
            .execute("DELETE FROM passwords WHERE id = ?1", params![id])?;
        Ok(())
    }

    /// Search for a password entry by service or username.
    ///
    /// # Arguments
    ///
    /// * `query` - The search query.
    ///
    /// # Returns
    ///
    /// A Result containing a vector of PasswordEntry objects or an error.
    ///
    /// # Errors
    ///
    /// If the query fails.
    pub fn search(&self, query: &str) -> Result<Vec<PasswordEntry>, Box<dyn std::error::Error>> {
        let mut stmt = self
            .connection
            .prepare("SELECT * FROM passwords WHERE service LIKE ?1 OR username LIKE ?1")?;

        let search_pattern = format!("%{}%", query);
        let entries = stmt.query_map(params![search_pattern], |row| {
            Ok(PasswordEntry {
                id: row.get(0)?,
                service: row.get(1)?,
                username: row.get(2)?,
                password: row.get(3)?,
                url: row.get(4)?,
                notes: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })?;

        Ok(entries.collect::<Result<Vec<_>, _>>()?)
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
        service: String,
        username: String,
        password: String,
        url: String,
        notes: String,
    ) -> Self {
        Self {
            id: None,
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
    use ring::rand::{SecureRandom, SystemRandom};

    use super::*;
    use crate::Encryption;

    fn create_test_encryption() -> Encryption {
        let mut salt = [0u8; 16];
        let rng = SystemRandom::new();
        rng.fill(&mut salt).unwrap();
        Encryption::new("test_password", &salt)
    }

    fn create_test_db() -> Database {
        let encryption = create_test_encryption();
        let db = Database {
            connection: Connection::open(":memory:").unwrap(),
            path: PathBuf::from(":memory:"),
            encryption,
        };

        db.connection
            .execute_batch(
                "
                    PRAGMA foreign_keys = ON;
                    PRAGMA journal_mode = WAL;
                    CREATE TABLE IF NOT EXISTS passwords (
                        id INTEGER PRIMARY KEY,
                        service TEXT NOT NULL,
                        username TEXT NOT NULL,
                        password TEXT NOT NULL,
                        url TEXT NOT NULL,
                        notes TEXT NOT NULL,
                        created_at TEXT NOT NULL,
                        updated_at TEXT NOT NULL
                    );
                ",
            )
            .unwrap();

        db
    }

    #[test]
    fn test_crud_operations() {
        let db = create_test_db();
        let entry = PasswordEntry {
            id: None,
            service: "test_service".to_string(),
            username: "test_user".to_string(),
            password: "test_pass".to_string(),
            url: "https://example.com".to_string(),
            notes: "test notes".to_string(),
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };

        // Test Create
        assert!(db.create(&entry).is_ok());

        // Test Read All
        let entries = db.read_all().unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].service, entry.service);
        assert_eq!(entries[0].password, entry.password);

        // Test Read by ID
        let id = entries[0].id.unwrap();
        let entry = db.read_by_id(id).unwrap();
        assert_eq!(entry.service, "test_service");

        // Test Update
        let mut updated_entry = entries[0].clone();
        updated_entry.service = "updated_service".to_string();
        let id = updated_entry.id.unwrap();
        db.update(id, updated_entry).unwrap();

        let updated_entries = db.read_all().unwrap();
        assert_eq!(updated_entries[0].service, "updated_service");

        // Test Search
        let search_results = db.search("updated").unwrap();
        assert_eq!(search_results.len(), 1);
        assert_eq!(search_results[0].service, "updated_service");

        // Test Delete
        let id = updated_entries[0].id.unwrap();
        db.delete(id).unwrap();
        let deleted_entries = db.read_all().unwrap();
        assert_eq!(deleted_entries.len(), 0);
    }
}
