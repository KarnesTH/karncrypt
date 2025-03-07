use tauri::State;

use crate::{commands::PasswordManagerState, Config};

#[tauri::command]
/// Get the auto logout time.
///
/// # Returns
///
/// A Result containing the auto logout time or an error.
///
/// # Errors
///
/// If the auto logout time cannot be retrieved.
pub async fn get_auto_logout_time() -> Result<u64, String> {
    let config = Config::load().map_err(|e| e.to_string())?;

    Ok(config.app.auto_logout_duration)
}

#[tauri::command(rename_all = "camelCase")]
/// Update the master password.
///
/// # Arguments
///
/// * `current_password` - The current master password.
/// * `new_password` - The new master password.
///
/// # Returns
///
/// A Result containing the completion status or an error.
///
/// # Errors
///
/// If the master password cannot be updated.
pub async fn update_master_password(
    state: State<'_, PasswordManagerState>,
    current_password: String,
    new_password: String,
) -> Result<(), String> {
    let mut state = state.0.lock().unwrap();
    match state.as_mut() {
        Some(pm) => match pm.update_master_password(&current_password, &new_password) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        },
        None => Err("Not logged in".into()),
    }
}

#[tauri::command]
pub async fn save_security_settings(auto_logout_duration: u64) -> Result<(), String> {
    let mut config = Config::load().map_err(|e| e.to_string())?;
    config.app.auto_logout_duration = auto_logout_duration;
    config.save().map_err(|e| e.to_string())?;
    Ok(())
}
