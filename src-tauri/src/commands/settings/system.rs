use tauri::AppHandle;
use tauri_plugin_opener::OpenerExt;

use crate::Config;

#[tauri::command]
/// Open the log folder in the file manager.
///
/// # Returns
///
/// A Result containing the completion status or an error.
///
/// # Errors
///
/// If the log folder cannot be opened.
pub async fn open_log_folder(app: AppHandle) -> Result<(), String> {
    let log_dir = Config::get_log_dir().map_err(|e| e.to_string())?;

    app.opener()
        .open_path(log_dir.to_string_lossy(), None::<&str>)
        .map_err(|e| e.to_string())?;

    Ok(())
}
