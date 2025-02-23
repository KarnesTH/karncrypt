use super::{database::User, PasswordEntry};

pub trait Model {
    fn table_name() -> &'static str;
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self, rusqlite::Error>
    where
        Self: Sized;
    fn to_params(&self) -> Vec<(&str, &dyn rusqlite::ToSql)>;
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
