mod commands;
mod password_manager;
mod utils;

use std::sync::Mutex;

use commands::PasswordManagerState;
use commands::{
    add_password, complete_setup, create_backup, delete_password, export_passwords,
    generate_password, get_database_settings, get_default_config, get_default_generator_length,
    get_passwords, import_passwords, login, logout, register, restore_backup,
    save_database_settings, update_password,
};

pub use password_manager::PasswordManager;

use tauri::AppHandle;
use tauri::Manager;
use tauri::State;
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_dialog::DialogExt;
pub use utils::Auth;
pub use utils::BackupManager;
pub use utils::Config;
pub use utils::Encryption;
pub use utils::TokenManager;
pub use utils::{Database, PasswordEntry};

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
/// Copy text to the clipboard.
///
/// # Arguments
///
/// * `text` - The text to copy to the clipboard.
///
/// # Returns
///
/// A Result containing the completion status or an error.
///
/// # Errors
///
/// If the text cannot be copied to the clipboard.
async fn copy_to_clipboard(app_handle: AppHandle, text: String) -> Result<(), String> {
    app_handle
        .clipboard()
        .write_text(text)
        .map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "camelCase")]
/// Verify the master password.
///
/// # Arguments
///
/// * `master_pass` - The master password to verify.
///
/// # Returns
///
/// A Result containing a boolean indicating if the master password is correct or an error.
///
/// # Errors
///
/// If the master password cannot be verified.
async fn verify_master_password(
    state: State<'_, PasswordManagerState>,
    master_pass: String,
) -> Result<bool, String> {
    let state = state.0.lock().unwrap();
    let pm = state.as_ref().ok_or("Not logged in")?;

    pm.verify_master_password(&master_pass)
        .map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "camelCase")]
/// Decrypt a password.
///
/// # Arguments
///
/// * `encrypted_password` - The encrypted password to decrypt.
///
/// # Returns
///
/// A Result containing the decrypted password or an error.
///
/// # Errors
///
/// If the password cannot be decrypted.
async fn decrypt_password(
    state: State<'_, PasswordManagerState>,
    encrypted_password: String,
) -> Result<String, String> {
    let state = state.0.lock().unwrap();
    let pm = state.as_ref().ok_or("Not logged in")?;

    pm.decrypt_password(&encrypted_password)
        .map_err(|e| e.to_string())
}

#[tauri::command]
/// Select a folder.
///
/// # Returns
///
/// A Result containing the selected folder path or an error.
///
/// # Errors
///
/// If no folder is selected.
async fn select_folder(app: AppHandle) -> Option<String> {
    let path = app.dialog().file().blocking_pick_folder();
    path.map(|p| p.as_path().unwrap().to_str().unwrap().to_string())
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
    PasswordManager::cleanup_on_startup().expect("error while cleaning up");
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .manage(PasswordManagerState(Mutex::new(None)))
        .on_window_event(|handle, event| match &event {
            tauri::WindowEvent::Destroyed => {
                if let Some(pm) = &*handle.state::<PasswordManagerState>().0.lock().unwrap() {
                    pm.cleanup_on_exit().expect("error during exit cleanup");
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            login,
            register,
            logout,
            generate_password,
            check_is_initialized,
            complete_setup,
            copy_to_clipboard,
            get_passwords,
            add_password,
            update_password,
            delete_password,
            verify_master_password,
            export_passwords,
            decrypt_password,
            import_passwords,
            select_folder,
            get_default_config,
            create_backup,
            restore_backup,
            get_database_settings,
            save_database_settings,
            get_default_generator_length
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
