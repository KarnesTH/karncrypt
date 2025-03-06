use log::{error, info};
use tauri::{AppHandle, State};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};

use crate::PasswordEntry;

pub use super::PasswordManagerState;

#[tauri::command]
/// Get all passwords.
///
/// # Returns
///
/// A Result containing a vector of all password entries or an error.
///
/// # Errors
///
/// If the passwords cannot be fetched.
pub async fn get_passwords(
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
/// Add a new password.
///
/// # Arguments
///
/// * `service` - The service for which the password is used.
/// * `username` - The username for the service.
/// * `password` - The password for the service.
/// * `url` - The URL of the service.
/// * `notes` - Optional notes for the password.
///
/// # Returns
///
/// A Result containing the completion status or an error.
pub async fn add_password(
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
/// Update a password.
///
/// # Arguments
///
/// * `id` - The ID of the password entry to update.
/// * `service` - The service for which the password is used.
/// * `username` - The username for the service.
/// * `password` - The password for the service.
/// * `url` - The URL of the service.
/// * `notes` - Optional notes for the password.
///
/// # Returns
///
/// A Result containing the completion status or an error.
///
/// # Errors
///
/// If the password cannot be updated.
pub async fn update_password(
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
/// Delete a password.
///
/// # Arguments
///
/// * `id` - The ID of the password entry to delete.
///
/// # Returns
///
/// A Result containing the completion status or an error.
///
/// # Errors
///
/// If the password cannot be deleted.
pub async fn delete_password(
    app_handle: AppHandle,
    state: State<'_, PasswordManagerState>,
    id: i32,
) -> Result<(), String> {
    let confirmed = app_handle
        .dialog()
        .message("Möchten Sie dieses Passwort wirklich löschen?")
        .title("Passwort löschen")
        .kind(MessageDialogKind::Info)
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
