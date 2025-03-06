mod password_manager;
mod utils;

use log::{error, info};
pub use password_manager::PasswordManager;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::AppHandle;
use tauri::Manager;
use tauri::State;
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_dialog::MessageDialogKind;
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};
pub use utils::Auth;
pub use utils::BackupManager;
pub use utils::Config;
pub use utils::Encryption;
pub use utils::TokenManager;
pub use utils::{Database, PasswordEntry};

struct PasswordManagerState(Mutex<Option<PasswordManager>>);

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
async fn register(
    app_handle: AppHandle,
    username: String,
    master_pass: String,
) -> Result<(), String> {
    let confirmed = app_handle
        .dialog()
        .message("Das Master-Passwort kann nicht wiederhergestellt werden. Bitte stellen Sie sicher, dass Sie es sicher aufbewahren.")
        .title("Warnung")
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
async fn login(
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
async fn logout(state: State<'_, PasswordManagerState>) -> Result<(), String> {
    if let Some(pm) = &*state.0.lock().unwrap() {
        pm.logout().map_err(|e| e.to_string())?;
    }
    *state.0.lock().unwrap() = None;
    Ok(())
}

#[tauri::command(rename_all = "camelCase")]
/// Generate a password.
///
/// # Arguments
///
/// * `length` - The length of the password to generate.
///
/// # Returns
///
/// A Result containing the generated password or an error.
///
/// # Errors
///
/// If the password cannot be generated.
async fn generate_password(length: usize) -> Result<String, String> {
    PasswordManager::generate_password(length).map_err(|e| e.to_string())
}

#[tauri::command]
/// Check if the app is initialized.
///
/// # Returns
///
/// A Result containing a boolean indicating if the app is initialized or an error.
async fn check_is_initialized() -> Result<bool, String> {
    let config = Config::load().map_err(|e| e.to_string())?;
    Ok(config.app.is_initialized)
}

#[derive(serde::Serialize)]
struct DefaultConfig {
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
async fn get_default_config() -> Result<DefaultConfig, String> {
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
async fn complete_setup(
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

#[tauri::command]
/// Copy text to the clipboard.
///
/// # Arguments
///
/// * `text` - The text to copy to the clipboard.
///
/// # Returns
///
/// A Result containing the completion status or an error.
///
/// # Errors
///
/// If the text cannot be copied to the clipboard.
async fn copy_to_clipboard(app_handle: AppHandle, text: String) -> Result<(), String> {
    app_handle
        .clipboard()
        .write_text(text)
        .map_err(|e| e.to_string())
}

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
async fn get_passwords(
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
async fn add_password(
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
async fn update_password(
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
async fn delete_password(
    app_handle: AppHandle,
    state: State<'_, PasswordManagerState>,
    id: i32,
) -> Result<(), String> {
    let confirmed = app_handle
        .dialog()
        .message("Möchten Sie dieses Passwort wirklich löschen?")
        .title("Passwort löschen")
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

#[tauri::command(rename_all = "camelCase")]
/// Verify the master password.
///
/// # Arguments
///
/// * `master_pass` - The master password to verify.
///
/// # Returns
///
/// A Result containing a boolean indicating if the master password is correct or an error.
///
/// # Errors
///
/// If the master password cannot be verified.
async fn verify_master_password(
    state: State<'_, PasswordManagerState>,
    master_pass: String,
) -> Result<bool, String> {
    let state = state.0.lock().unwrap();
    let pm = state.as_ref().ok_or("Not logged in")?;

    pm.verify_master_password(&master_pass)
        .map_err(|e| e.to_string())
}

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
async fn export_passwords(
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
async fn import_passwords(
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
            match bm.import_csv(&path) {
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
                    return Err(e.to_string());
                }
            }
        }
        None => Ok("Import abgebrochen".to_string()),
    }
}

#[tauri::command(rename_all = "camelCase")]
/// Decrypt a password.
///
/// # Arguments
///
/// * `encrypted_password` - The encrypted password to decrypt.
///
/// # Returns
///
/// A Result containing the decrypted password or an error.
///
/// # Errors
///
/// If the password cannot be decrypted.
async fn decrypt_password(
    state: State<'_, PasswordManagerState>,
    encrypted_password: String,
) -> Result<String, String> {
    let state = state.0.lock().unwrap();
    let pm = state.as_ref().ok_or("Not logged in")?;

    pm.decrypt_password(&encrypted_password)
        .map_err(|e| e.to_string())
}

#[tauri::command]
/// Select a folder.
///
/// # Returns
///
/// A Result containing the selected folder path or an error.
///
/// # Errors
///
/// If no folder is selected.
async fn select_folder(app: AppHandle) -> Option<String> {
    let path = app.dialog().file().blocking_pick_folder();
    path.map(|p| p.as_path().unwrap().to_str().unwrap().to_string())
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
async fn create_backup(
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
async fn restore_backup(app: AppHandle, master_pass: String) -> Result<(), String> {
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
struct DatabaseSettingsConfig {
    db_path: String,
    db_name: String,
    auto_backup: bool,
    backup_interval: String,
    max_backups: usize,
    backup_path: String,
    export_path: String,
}

#[tauri::command]
async fn get_database_settings() -> Result<DatabaseSettingsConfig, String> {
    let config = Config::load().map_err(|e| e.to_string())?;

    let backup_interval = match config.backup.interval {
        utils::BackupInterval::Daily => "Täglich",
        utils::BackupInterval::Weekly => "Wöchentlich",
        utils::BackupInterval::Monthly => "Monatlich",
        utils::BackupInterval::Yearly => "Jährlich",
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
async fn save_database_settings(
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
        "Täglich" => utils::BackupInterval::Daily,
        "Wöchentlich" => utils::BackupInterval::Weekly,
        "Monatlich" => utils::BackupInterval::Monthly,
        "Jährlich" => utils::BackupInterval::Yearly,
        _ => return Err("Ungültiges Backup-Intervall".into()),
    };
    config.backup.max_backups = max_backups;
    config.backup.backup_path = PathBuf::from(backup_path);
    config.backup.export_path = PathBuf::from(export_path);

    config.save().map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
/// Run the Tauri application.
///
/// This function sets up the logger and runs the Tauri application.
pub fn run() {
    let config = Config::load().expect("error while loading config");
    config
        .setup_logger()
        .expect("error while setting up logger");
    PasswordManager::cleanup_on_startup().expect("error while cleaning up");
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .manage(PasswordManagerState(Mutex::new(None)))
        .on_window_event(|handle, event| match &event {
            tauri::WindowEvent::Destroyed => {
                if let Some(pm) = &*handle.state::<PasswordManagerState>().0.lock().unwrap() {
                    pm.cleanup_on_exit().expect("error during exit cleanup");
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            login,
            register,
            logout,
            generate_password,
            check_is_initialized,
            complete_setup,
            copy_to_clipboard,
            get_passwords,
            add_password,
            update_password,
            delete_password,
            verify_master_password,
            export_passwords,
            decrypt_password,
            import_passwords,
            select_folder,
            get_default_config,
            create_backup,
            restore_backup,
            get_database_settings,
            save_database_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
