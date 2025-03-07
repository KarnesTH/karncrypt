use crate::Config;

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
