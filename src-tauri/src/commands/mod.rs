mod auth;
mod generator;
mod manager;
mod settings;
mod setup;

pub use auth::{login, logout, register};
pub use generator::generate_password;
pub use manager::{add_password, delete_password, get_passwords, update_password};
pub use settings::{
    check_update, create_backup, export_passwords, get_auto_logout_time, get_database_settings,
    get_default_generator_length, import_passwords, is_autostart_enabled, open_log_folder,
    restore_backup, save_app_settings, save_database_settings, save_security_settings,
    toggle_autostart, update_master_password,
};

pub use setup::{complete_setup, get_default_config};

use std::sync::Mutex;

use crate::PasswordManager;

pub struct PasswordManagerState(pub Mutex<Option<PasswordManager>>);
