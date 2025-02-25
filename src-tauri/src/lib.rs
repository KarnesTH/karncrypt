mod password_manager;
mod utils;

pub use password_manager::PasswordManager;
// use tauri::AppHandle;
// use tauri_plugin_dialog::DialogExt;
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

#[tauri::command(rename_all = "camelCase")]
/// Generate a password.
///
/// # Arguments
///
/// * `length` - The length of the password to generate.
///
/// # Returns
///
/// A Result containing the generated password or an error.
///
/// # Errors
///
/// If the password cannot be generated.
async fn generate_password(length: usize) -> Result<String, String> {
    PasswordManager::generate_password(length).map_err(|e| e.to_string())
}

#[tauri::command]
async fn check_is_initialized() -> Result<bool, String> {
    let config = Config::load().map_err(|e| e.to_string())?;
    Ok(config.app.is_initialized)
}

#[tauri::command]
async fn complete_setup(custom_path: Option<String>) -> Result<(), String> {
    let mut config = Config::load().map_err(|e| e.to_string())?;
    config.app.is_initialized = true;
    config.app.db_custom_path = custom_path;
    config.save().map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
/// Run the Tauri application.
///
/// This function sets up the logger and runs the Tauri application.
pub fn run() {
    let config = Config::load().expect("error while loading config");
    config
        .setup_logger()
        .expect("error while setting up logger");
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            login,
            register,
            generate_password,
            check_is_initialized,
            complete_setup
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
