use super::{database::User, PasswordEntry};

pub trait Model {
    /// Get the name of the table for the model.
    ///
    /// # Returns
    ///
    /// The name of the table for the model.
    fn table_name() -> &'static str;

    /// Create a new instance of the model from a database row.
    ///
    /// # Arguments
    ///
    /// * `row` - The database row to create the model from.
    ///
    /// # Returns
    ///
    /// A new instance of the model.
    ///
    /// # Errors
    ///
    /// If the model cannot be created.
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self, rusqlite::Error>
    where
        Self: Sized;

    /// Get the parameters for the model.
    ///
    /// # Returns
    ///
    /// A vector of tuples containing the parameter name and value.
    fn to_params(&self) -> Vec<(&str, &dyn rusqlite::ToSql)>;

    /// Get the ID of the model.
    ///
    /// # Returns
    ///
    /// The ID of the model if it exists.
    fn get_id(&self) -> Option<i32>;
}

impl Model for User {
    fn table_name() -> &'static str {
        "user"
    }

    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self, rusqlite::Error> {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            master_key: row.get(2)?,
            created_at: row.get(3)?,
            last_login: row.get(4)?,
        })
    }

    fn to_params(&self) -> Vec<(&str, &dyn rusqlite::ToSql)> {
        vec![
            ("username", &self.username),
            ("master_key", &self.master_key),
            ("created_at", &self.created_at),
            ("last_login", &self.last_login),
        ]
    }

    fn get_id(&self) -> Option<i32> {
        self.id
    }
}

impl Model for PasswordEntry {
    fn table_name() -> &'static str {
        "passwords"
    }

    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self, rusqlite::Error> {
        Ok(PasswordEntry {
            id: row.get(0)?,
            user_id: row.get(1)?,
            service: row.get(2)?,
            username: row.get(3)?,
            password: row.get(4)?,
            url: row.get(5)?,
            notes: row.get(6)?,
            created_at: row.get(7)?,
            updated_at: row.get(8)?,
        })
    }

    fn to_params(&self) -> Vec<(&str, &dyn rusqlite::ToSql)> {
        vec![
            ("user_id", &self.user_id),
            ("service", &self.service),
            ("username", &self.username),
            ("password", &self.password),
            ("url", &self.url),
            ("notes", &self.notes),
            ("created_at", &self.created_at),
            ("updated_at", &self.updated_at),
        ]
    }

    fn get_id(&self) -> Option<i32> {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_user_model() {
        assert_eq!(User::table_name(), "user");

        let user = User {
            id: Some(1),
            username: "test".to_string(),
            master_key: vec![1, 2, 3],
            created_at: Utc::now().to_rfc3339(),
            last_login: Utc::now().to_rfc3339(),
        };
        assert_eq!(user.get_id(), Some(1));

        let params = user.to_params();
        assert_eq!(params.len(), 4);
        assert_eq!(params[0].0, "username");
        assert_eq!(params[1].0, "master_key");
        assert_eq!(params[2].0, "created_at");
        assert_eq!(params[3].0, "last_login");
    }

    #[test]
    fn test_password_entry_model() {
        assert_eq!(PasswordEntry::table_name(), "passwords");

        let entry = PasswordEntry {
            id: Some(1),
            user_id: 1,
            service: "test".to_string(),
            username: "user".to_string(),
            password: "pass".to_string(),
            url: "url".to_string(),
            notes: "notes".to_string(),
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };
        assert_eq!(entry.get_id(), Some(1));

        let params = entry.to_params();
        assert_eq!(params.len(), 8);
        assert_eq!(params[0].0, "user_id");
        assert_eq!(params[1].0, "service");
        assert_eq!(params[2].0, "username");
        assert_eq!(params[3].0, "password");
        assert_eq!(params[4].0, "url");
        assert_eq!(params[5].0, "notes");
        assert_eq!(params[6].0, "created_at");
        assert_eq!(params[7].0, "updated_at");
    }

    #[test]
    fn test_null_id() {
        let user = User {
            id: None,
            username: "test".to_string(),
            master_key: vec![1, 2, 3],
            created_at: Utc::now().to_rfc3339(),
            last_login: Utc::now().to_rfc3339(),
        };
        assert_eq!(user.get_id(), None);

        let entry = PasswordEntry {
            id: None,
            user_id: 1,
            service: "test".to_string(),
            username: "user".to_string(),
            password: "pass".to_string(),
            url: "url".to_string(),
            notes: "notes".to_string(),
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };
        assert_eq!(entry.get_id(), None);
    }
}
