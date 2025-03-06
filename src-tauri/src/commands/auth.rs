pub use super::PasswordManagerState;
use crate::Auth;
use crate::PasswordManager;
use log::{error, info};
use tauri::AppHandle;
use tauri::State;
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};

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
pub async fn register(
    app_handle: AppHandle,
    username: String,
    master_pass: String,
) -> Result<(), String> {
    let confirmed = app_handle
        .dialog()
        .message("Das Master-Passwort kann nicht wiederhergestellt werden. Bitte stellen Sie sicher, dass Sie es sicher aufbewahren.")
        .title("Warnung")
        .kind(MessageDialogKind::Warning)
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
pub async fn login(
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
/// Logout the current user.
///
/// # Returns
///
/// A Result containing the completion status or an error.
///
/// # Errors
///
/// If the user cannot be logged out.
pub async fn logout(state: State<'_, PasswordManagerState>) -> Result<(), String> {
    if let Some(pm) = &*state.0.lock().unwrap() {
        pm.logout().map_err(|e| e.to_string())?;
    }
    *state.0.lock().unwrap() = None;
    Ok(())
}
