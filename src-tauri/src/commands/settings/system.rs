use log::info;
use tauri::AppHandle;
use tauri_plugin_opener::OpenerExt;
use tauri_plugin_updater::UpdaterExt;

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

#[tauri::command]
/// Check for updates and install them if available.
///
/// # Returns
///
/// A Result containing the completion status or an error.
///
/// # Errors
///
/// If the updater fails to check for updates or install them.
pub async fn check_update(app: AppHandle) -> tauri_plugin_updater::Result<()> {
    if let Some(update) = app.updater()?.check().await? {
        let mut downloaded = 0;

        update
            .download_and_install(
                |chunk_length, content_length| {
                    downloaded += chunk_length;
                    info!("downloaded {downloaded} from {content_length:?}");
                },
                || {
                    info!("download finished");
                },
            )
            .await?;

        info!("update installed");
        app.restart();
    }

    Ok(())
}
