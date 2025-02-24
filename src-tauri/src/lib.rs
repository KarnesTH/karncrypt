mod password_manager;
mod utils;

pub use password_manager::PasswordManager;
pub use utils::Auth;
pub use utils::Config;
pub use utils::Encryption;
pub use utils::{Database, PasswordEntry};

#[tauri::command(rename_all = "camelCase")]
/// Register a new user.
///
/// # Arguments
///
/// * `username` - The username to register.
/// * `master_pass` - The master password to use for the database.
///
/// # Returns
///
/// A Result containing the new user's ID or an error.
///
/// # Errors
///
/// If the user cannot be registered.
async fn register(username: String, master_pass: String) -> Result<(), String> {
    let pm = PasswordManager::new(&master_pass).map_err(|e| e.to_string())?;

    let auth = Auth::new(pm.db);
    auth.register(&username, &master_pass)
        .map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "camelCase")]
/// Login a user.
///
/// # Arguments
///
/// * `username` - The username to login.
/// * `master_pass` - The master password to use for the database.
///
/// # Returns
///
/// A Result containing the user's ID or an error.
///
/// # Errors
///
/// If the user cannot be logged in.
async fn login(username: String, master_pass: String) -> Result<i32, String> {
    let pm = PasswordManager::new(&master_pass).map_err(|e| e.to_string())?;

    let auth = Auth::new(pm.db);
    auth.login(&username, &master_pass)
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let config = Config::load().expect("error while loading config");
    config
        .setup_logger()
        .expect("error while setting up logger");
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![login, register])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
