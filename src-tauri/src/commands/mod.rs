mod auth;
mod generator;
mod manager;
mod settings;
mod setup;

pub use auth::{login, logout, register};
pub use generator::generate_password;
pub use manager::{add_password, delete_password, get_passwords, update_password};
pub use settings::{
    create_backup, export_passwords, get_database_settings, get_default_generator_length,
    import_passwords, restore_backup, save_app_settings, save_database_settings,
};

pub use setup::{complete_setup, get_default_config};

use std::sync::Mutex;

use crate::PasswordManager;

pub struct PasswordManagerState(pub Mutex<Option<PasswordManager>>);
