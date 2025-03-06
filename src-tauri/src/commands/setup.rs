use std::path::PathBuf;

use log::info;

use crate::Config;

#[derive(serde::Serialize)]
pub struct DefaultConfig {
    pub db_name: String,
    pub db_path: String,
    pub backup_path: String,
}

#[tauri::command(rename_all = "camelCase")]
/// Get the default configuration.
///
/// # Returns
///
/// A Result containing the default configuration or an error.
///
/// # Errors
///
/// If the default configuration cannot be fetched.
pub async fn get_default_config() -> Result<DefaultConfig, String> {
    let config = Config::load().unwrap();

    Ok(DefaultConfig {
        db_name: config.database.db_name.to_string(),
        db_path: config.database.db_path.to_string_lossy().to_string(),
        backup_path: config.backup.backup_path.to_string_lossy().to_string(),
    })
}

#[tauri::command(rename_all = "camelCase")]
/// Complete the app setup.
///
/// # Arguments
///
/// * `custom_path` - An optional custom path for the database.
///
/// # Returns
///
/// A Result containing the completion status or an error.
pub async fn complete_setup(
    db_path: String,
    db_name: String,
    backup_path: String,
) -> Result<(), String> {
    info!("Complete Setup called!");
    let mut config = Config::load().unwrap();
    config.app.is_initialized = true;
    config.database.db_name = db_name;
    config.database.db_path = PathBuf::from(db_path);
    config.backup.backup_path = PathBuf::from(backup_path);
    config.save().map_err(|e| e.to_string())?;

    Ok(())
}
