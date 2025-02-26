mod password_manager;
mod utils;

use log::{error, info};
pub use password_manager::PasswordManager;
use std::sync::Mutex;
use tauri::AppHandle;
use tauri::Manager;
use tauri::State;
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};
pub use utils::Auth;
pub use utils::Config;
pub use utils::Encryption;
pub use utils::TokenManager;
pub use utils::{Database, PasswordEntry};

struct PasswordManagerState(Mutex<Option<PasswordManager>>);

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
        match PasswordManager::new(&master_pass) {
            Ok(pm) => {
                let auth = Auth::new(&pm.db);
                match auth.register(&username, &master_pass) {
                    Ok(_) => {
                        info!("Successfully registered user: {}", username);
                        Ok(())
                    }
                    Err(e) => {
                        error!("Failed to register user {}: {}", username, e);
                        Err(e.to_string())
                    }
                }
            }
            Err(e) => {
                error!(
                    "Failed to create PasswordManager during registration: {}",
                    e
                );
                Err(e.to_string())
            }
        }
    } else {
        info!("Registration cancelled by user");
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
async fn login(
    state: State<'_, PasswordManagerState>,
    username: String,
    master_pass: String,
) -> Result<(), String> {
    info!("Login attempt for user: {}", username);

    let mut pm = PasswordManager::new(&master_pass).map_err(|e| {
        error!("Failed to create PasswordManager: {}", e);
        e.to_string()
    })?;

    match pm.login(&username, &master_pass) {
        Ok(_) => {
            log::info!("Login successful");
            *state.0.lock().unwrap() = Some(pm);
            Ok(())
        }
        Err(e) => {
            log::error!("Login failed: {}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
async fn logout(state: State<'_, PasswordManagerState>) -> Result<(), String> {
    if let Some(pm) = &*state.0.lock().unwrap() {
        pm.logout().map_err(|e| e.to_string())?;
    }
    *state.0.lock().unwrap() = None;
    Ok(())
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

#[tauri::command]
async fn get_passwords(
    state: State<'_, PasswordManagerState>,
) -> Result<Vec<PasswordEntry>, String> {
    info!("Fetching passwords");
    let state = state.0.lock().unwrap();
    match state.as_ref() {
        Some(pm) => match pm.get_passwords() {
            Ok(passwords) => {
                info!("Successfully fetched {} passwords", passwords.len());
                Ok(passwords)
            }
            Err(e) => {
                error!("Failed to fetch passwords: {}", e);
                Err(e.to_string())
            }
        },
        None => {
            error!("Attempted to fetch passwords without being logged in");
            Err("Not logged in".into())
        }
    }
}

#[tauri::command]
async fn add_password(
    state: State<'_, PasswordManagerState>,
    service: String,
    username: String,
    password: String,
    url: String,
    notes: Option<String>,
) -> Result<(), String> {
    info!("Adding new password entry for service: {}", service);
    let state = state.0.lock().unwrap();
    match state.as_ref() {
        Some(pm) => match pm.add_password(service.clone(), username, password, url, notes) {
            Ok(_) => {
                info!("Successfully added password for service: {}", service);
                Ok(())
            }
            Err(e) => {
                error!("Failed to add password for service {}: {}", service, e);
                Err(e.to_string())
            }
        },
        None => {
            error!("Attempted to add password without being logged in");
            Err("Not logged in".into())
        }
    }
}

#[tauri::command]
async fn update_password(
    state: State<'_, PasswordManagerState>,
    id: i32,
    service: String,
    username: String,
    password: String,
    url: String,
    notes: Option<String>,
) -> Result<(), String> {
    info!("Updating password entry {} for service: {}", id, service);
    let state = state.0.lock().unwrap();
    match state.as_ref() {
        Some(pm) => match pm.update_password(id, service.clone(), username, password, url, notes) {
            Ok(_) => {
                info!(
                    "Successfully updated password {} for service: {}",
                    id, service
                );
                Ok(())
            }
            Err(e) => {
                error!(
                    "Failed to update password {} for service {}: {}",
                    id, service, e
                );
                Err(e.to_string())
            }
        },
        None => {
            error!("Attempted to update password without being logged in");
            Err("Not logged in".into())
        }
    }
}

#[tauri::command]
async fn delete_password(
    app_handle: AppHandle,
    state: State<'_, PasswordManagerState>,
    id: i32,
) -> Result<(), String> {
    let confirmed = app_handle
        .dialog()
        .message("Möchten Sie dieses Passwort wirklich löschen?")
        .title("Passwort löschen")
        .buttons(MessageDialogButtons::YesNo)
        .blocking_show();

    if confirmed {
        let state = state.0.lock().unwrap();
        match state.as_ref() {
            Some(pm) => match pm.delete_password(id) {
                Ok(_) => {
                    info!("Successfully deleted password entry: {}", id);
                    Ok(())
                }
                Err(e) => {
                    error!("Failed to delete password entry {}: {}", id, e);
                    Err(e.to_string())
                }
            },
            None => {
                error!("Attempted to delete password without being logged in");
                Err("Not logged in".into())
            }
        }
    } else {
        info!("Password deletion cancelled by user");
        Err("Löschen abgebrochen".into())
    }
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
        .on_window_event(|handle, event| {
            if let tauri::WindowEvent::Destroyed = event {
                if let Some(pm) = &*handle.state::<PasswordManagerState>().0.lock().unwrap() {
                    pm.cleanup_on_exit().expect("error during exit cleanup");
                }
            }
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
            verify_master_password
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
