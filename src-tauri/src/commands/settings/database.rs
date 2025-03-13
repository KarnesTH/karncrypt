use std::{fs, path::PathBuf};

use log::{error, info};
use tauri::{AppHandle, State};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};

use crate::{commands::PasswordManagerState, BackupManager, Config};

#[tauri::command(rename_all = "camelCase")]
/// Export all passwords to a CSV file.
///
/// # Returns
///
/// A Result containing the completion status or an error.
///
/// # Errors
///
/// If the passwords cannot be exported.
pub async fn export_passwords(
    app: AppHandle,
    state: State<'_, PasswordManagerState>,
) -> Result<(), String> {
    let state = state.0.lock().unwrap();
    let pm = state.as_ref().ok_or("Not logged in")?;

    let config = Config::load().map_err(|e| e.to_string())?;

    let confirmed = app
        .dialog()
        .message(
            "Achtung: Die exportierte CSV-Datei wird deine Passwörter im Klartext enthalten.\n\n\
            Bitte beachte:\n\
            • Bewahre diese Datei sicher auf\n\
            • Lösche die Datei nach dem Import in ein anderes System\n\
            • Schütze die Datei ggf. mit einem zusätzlichen Passwort\n\
            • Teile diese Datei niemals unverschlüsselt\n\n\
            Möchtest du trotzdem fortfahren?",
        )
        .title("Sicherheitswarnung - Export")
        .kind(MessageDialogKind::Warning)
        .buttons(MessageDialogButtons::YesNo)
        .blocking_show();

    if confirmed {
        let path = config.backup.export_path;

        if !path.exists() {
            let confirm = app
                .dialog()
                .message(format!(
                    "Der Export-Ordner existiert noch nicht.\n\n\
                    Folgender Ordner wird benötigt:\n\
                    {}\n\n\
                    Ordner jetzt anlegen?",
                    path.display()
                ))
                .title("Export-Ordner erstellen")
                .kind(MessageDialogKind::Warning)
                .buttons(MessageDialogButtons::OkCancel)
                .blocking_show();

            if confirm {
                fs::create_dir_all(&path).map_err(|e| e.to_string())?;
            } else {
                return Err("Du hast die Erstellung des Export-Ordners abgebrochen.".into());
            }
        }

        let bm = BackupManager::new(&pm.db);
        bm.export_csv(&path).map_err(|e| e.to_string())?;

        Ok(())
    } else {
        info!("Export cancelled by user");
        Err("Export wurde abgebrochen!".into())
    }
}

#[tauri::command]
/// Import passwords from a CSV file.
///
/// # Returns
///
/// A Result containing the completion status or an error.
///
/// # Errors
///
/// If the passwords cannot be imported.
pub async fn import_passwords(
    app: AppHandle,
    state: State<'_, PasswordManagerState>,
) -> Result<String, String> {
    let state = state.0.lock().unwrap();
    let pm = state.as_ref().ok_or("Not logged in")?;

    let file_path = app.dialog().file().blocking_pick_file();

    match file_path {
        Some(path) => {
            let path = path.as_path().unwrap();
            let bm = BackupManager::new(&pm.db);
            match bm.import_csv(path) {
                Ok(result) => {
                    app.dialog()
                        .message(format!(
                            "Import erfolgreich:\n
                            {} Einträge importiert\n
                            {} Dublikate überstrungen\n
                            {} Fehlerhaft",
                            result.imported,
                            result.skipped,
                            result.errors.len()
                        ))
                        .title("Import abgeschlossen")
                        .kind(MessageDialogKind::Info)
                        .blocking_show();
                    Ok("Import erfolgreich".to_string())
                }
                Err(e) => {
                    error!("Failed to import passwords: {}", e);
                    Err(e.to_string())
                }
            }
        }
        None => Ok("Import abgebrochen".to_string()),
    }
}

#[tauri::command]
/// Create a backup.
///
/// # Arguments
///
/// * `master_pass` - The master password to use for the backup.
///
/// # Returns
///
/// A Result containing the completion status or an error.
///
/// # Errors
///
/// If the backup cannot be created.
pub async fn create_backup(
    app: AppHandle,
    state: State<'_, PasswordManagerState>,
    master_pass: String,
) -> Result<(), String> {
    let state = state.0.lock().unwrap();
    let pm = state.as_ref().ok_or("Not logged in")?;

    let config = Config::load().map_err(|e| e.to_string())?;
    let config_dir = Config::get_config_dir().map_err(|e| e.to_string())?;
    let backup_path = config.backup.backup_path;

    if !backup_path.exists() {
        let confirm = app
            .dialog()
            .message(format!(
                "Der Backup-Ordner existiert noch nicht.\n\n\
                Folgender Ordner wird benötigt:\n\
                {}\n\n\
                Ordner jetzt anlegen?",
                backup_path.display()
            ))
            .title("Backup-Ordner erstellen")
            .kind(MessageDialogKind::Warning)
            .buttons(MessageDialogButtons::OkCancel)
            .blocking_show();

        if confirm {
            fs::create_dir_all(&backup_path).map_err(|e| e.to_string())?;
        } else {
            return Err("Du hast die Erstellung des Backup-Ordners abgebrochen.".into());
        }
    }

    let bm = BackupManager::new(&pm.db);
    bm.create_backup(&backup_path, &config_dir, &master_pass)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
/// Restore a backup.
///
/// # Arguments
///
/// * `master_pass` - The master password to use for the backup.
///
/// # Returns
///
/// A Result containing the completion status or an error.
///
/// # Errors
///
/// If the backup cannot be restored.
pub async fn restore_backup(app: AppHandle, master_pass: String) -> Result<(), String> {
    let backup_file = app
        .dialog()
        .file()
        .add_filter("KarnCrypt Backup", &["pmbackup"])
        .blocking_pick_file();

    let backup_path = match backup_file {
        Some(path) => path.as_path().unwrap().to_path_buf(),
        None => return Err("Keine Backup-Datei ausgewählt".into()),
    };

    let config_dir = Config::get_config_dir().map_err(|e| e.to_string())?;

    BackupManager::restore_backup(&backup_path, &config_dir, &master_pass)
        .map_err(|e| e.to_string())?;

    app.dialog()
        .message("Backup erfolgreich wiederhergestellt")
        .title("Backup wiederherstellen")
        .kind(MessageDialogKind::Info)
        .buttons(MessageDialogButtons::Ok)
        .blocking_show();

    app.restart();
}

#[derive(serde::Serialize)]
pub struct DatabaseSettingsConfig {
    db_path: String,
    db_name: String,
    auto_backup: bool,
    backup_interval: String,
    max_backups: usize,
    backup_path: String,
    export_path: String,
}

#[tauri::command]
/// Get the database settings.
///
/// # Returns
///
/// A Result containing the database settings or an error.
///
/// # Errors
///
/// If the settings cannot be fetched.
pub async fn get_database_settings() -> Result<DatabaseSettingsConfig, String> {
    let config = Config::load().map_err(|e| e.to_string())?;

    let backup_interval = match config.backup.interval {
        crate::utils::BackupInterval::Daily => "Täglich",
        crate::utils::BackupInterval::Weekly => "Wöchentlich",
        crate::utils::BackupInterval::Monthly => "Monatlich",
        crate::utils::BackupInterval::Yearly => "Jährlich",
    };

    Ok(DatabaseSettingsConfig {
        db_name: config.database.db_name.to_string(),
        db_path: config.database.db_path.to_string_lossy().to_string(),
        auto_backup: config.backup.enabled,
        backup_interval: backup_interval.to_string(),
        max_backups: config.backup.max_backups,
        backup_path: config.backup.backup_path.to_string_lossy().to_string(),
        export_path: config.backup.export_path.to_string_lossy().to_string(),
    })
}

#[tauri::command(rename_all = "camelCase")]
/// Save the database settings.
///
/// # Arguments
///
/// * `db_name` - The name of the database.
/// * `db_path` - The path to the database.
/// * `auto_backup` - Whether to enable automatic backups.
/// * `backup_interval` - The interval at which to create backups.
/// * `max_backups` - The maximum number of backups to keep.
/// * `backup_path` - The path to store backups.
/// * `export_path` - The path to store exported passwords.
///
/// # Returns
///
/// A Result containing the completion status or an error.
///
/// # Errors
///
/// If the settings cannot be saved.
pub async fn save_database_settings(
    db_name: String,
    db_path: String,
    auto_backup: bool,
    backup_interval: String,
    max_backups: usize,
    backup_path: String,
    export_path: String,
) -> Result<(), String> {
    let mut config = Config::load().map_err(|e| e.to_string())?;

    config.database.db_name = db_name;
    config.database.db_path = PathBuf::from(db_path);
    config.backup.enabled = auto_backup;
    config.backup.interval = match backup_interval.as_str() {
        "Täglich" => crate::utils::BackupInterval::Daily,
        "Wöchentlich" => crate::utils::BackupInterval::Weekly,
        "Monatlich" => crate::utils::BackupInterval::Monthly,
        "Jährlich" => crate::utils::BackupInterval::Yearly,
        _ => return Err("Ungültiges Backup-Intervall".into()),
    };
    config.backup.max_backups = max_backups;
    config.backup.backup_path = PathBuf::from(backup_path);
    config.backup.export_path = PathBuf::from(export_path);

    config.save().map_err(|e| e.to_string())?;

    Ok(())
}
