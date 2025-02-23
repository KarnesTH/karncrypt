use std::path::PathBuf;

use chrono::Utc;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

use crate::utils::Encryption;

use super::models::Model;

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub master_key: Vec<u8>,
    pub created_at: String,
    pub last_login: String,
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
                    updated_at TEXT NOT NULL
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
        self.connection.execute(&query, &values[..])?;

        Ok(())
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
        let mut stmt = self
            .connection
            .prepare(&format!("SELECT * FROM {}", T::table_name()))?;
        let entries = stmt.query_map([], |row| T::from_row(row))?;

        let mut result = Vec::new();
        for entry in entries {
            result.push(entry?);
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

        self.connection.execute(&query, &param_values[..])?;

        Ok(())
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
        self.connection.execute(
            &format!("DELETE FROM {} WHERE id = ?1", T::table_name()),
            [id],
        )?;

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
