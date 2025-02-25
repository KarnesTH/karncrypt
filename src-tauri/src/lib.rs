mod password_manager;
mod utils;

pub use password_manager::PasswordManager;
use tauri::AppHandle;
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};
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
async fn register(
    app_handle: AppHandle,
    username: String,
    master_pass: String,
) -> Result<(), String> {
    let confirmed = app_handle
        .dialog()
        .message("Das Master-Passwort kann nicht wiederhergestellt werden. Bitte stellen Sie sicher, dass Sie es sicher aufbewahren.")
        .title("Warnung")
        .buttons(MessageDialogButtons::OkCancel)
        .blocking_show();

    if confirmed {
        let pm = PasswordManager::new(&master_pass).map_err(|e| e.to_string())?;
        let auth = Auth::new(pm.db);
        auth.register(&username, &master_pass)
            .map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Setup abgebrochen".to_string())
    }
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
/// Check if the app is initialized.
///
/// # Returns
///
/// A Result containing a boolean indicating if the app is initialized or an error.
async fn check_is_initialized() -> Result<bool, String> {
    let config = Config::load().map_err(|e| e.to_string())?;
    Ok(config.app.is_initialized)
}

#[tauri::command]
/// Complete the app setup.
///
/// # Arguments
///
/// * `custom_path` - An optional custom path for the database.
///
/// # Returns
///
/// A Result containing the completion status or an error.
async fn complete_setup(custom_path: Option<String>) -> Result<(), String> {
    let mut config = Config::load().map_err(|e| e.to_string())?;
    config.app.is_initialized = true;
    config.app.db_custom_path = custom_path;
    config.save().map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn copy_to_clipboard(app_handle: AppHandle, text: String) -> Result<(), String> {
    app_handle
        .clipboard()
        .write_text(text)
        .map_err(|e| e.to_string())
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
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            login,
            register,
            generate_password,
            check_is_initialized,
            complete_setup,
            copy_to_clipboard
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
