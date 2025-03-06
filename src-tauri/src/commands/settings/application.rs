use serde::Serialize;

use crate::Config;

#[derive(Serialize)]
pub struct AppSettingsConfig {
    default_length: usize,
}

#[tauri::command]
/// Get the default generator length.
///
/// # Returns
///
/// A Result containing the default generator length or an error.
///
/// # Errors
///
/// If the default generator length cannot be fetched.
pub async fn get_default_generator_length() -> Result<AppSettingsConfig, String> {
    let config = Config::load().map_err(|e| e.to_string())?;

    Ok(AppSettingsConfig {
        default_length: config.generator.default_length,
    })
}
