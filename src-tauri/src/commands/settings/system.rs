use log::info;
use tauri::AppHandle;
use tauri_plugin_autostart::ManagerExt;
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

#[tauri::command]
/// Toggle the autostart setting.
///
/// # Arguments
///
/// * `enable` - A boolean indicating if autostart should be enabled.
///
/// # Returns
///
/// A Result containing the completion status or an error.
///
/// # Errors
///
/// If the autostart setting cannot be toggled.
pub async fn toggle_autostart(app: AppHandle, enable: bool) -> Result<(), String> {
    #[cfg(desktop)]
    {
        if enable {
            app.autolaunch().enable().map_err(|e| e.to_string())?;
        } else {
            app.autolaunch().disable().map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

#[tauri::command]
/// Check if autostart is enabled.
///
/// # Returns
///
/// A Result containing a boolean indicating if autostart is enabled or an error.
///
/// # Errors
///
/// If the autostart setting cannot be checked.
pub async fn is_autostart_enabled(app: AppHandle) -> Result<bool, String> {
    #[cfg(desktop)]
    {
        app.autolaunch().is_enabled().map_err(|e| e.to_string())
    }
    #[cfg(not(desktop))]
    {
        Ok(false)
    }
}
