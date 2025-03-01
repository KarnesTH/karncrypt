use crate::utils::{Database, Encryption};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use chrono::Utc;
use log::info;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use super::PasswordEntry;
use super::{BackupCompressor, BackupFile};

pub struct ImportResult {
    pub imported: usize,
    pub skipped: usize,
    pub errors: Vec<String>,
}

pub struct BackupManager {
    encryption: Encryption,
}

impl BackupManager {
    /// Create a new BackupManager
    ///
    /// # Arguments
    ///
    /// * `encryption` - The encryption engine to use for the backup
    ///
    /// # Returns
    ///
    /// A new BackupManager instance
    pub fn new(encryption: Encryption) -> Self {
        Self { encryption }
    }

    /// Create a backup of the database and configuration files
    ///
    /// # Arguments
    ///
    /// * `db` - The database to backup
    /// * `backup_path` - The directory to store the backup
    /// * `config_dir` - The directory containing the configuration files
    /// * `master_password` - The master password to encrypt the backup
    ///
    /// # Returns
    ///
    /// The path to the created backup file
    ///
    /// # Errors
    ///
    /// Returns an error if the backup creation fails
    pub fn create_backup(
        &self,
        db: &Database,
        backup_path: &Path,
        config_dir: &Path,
        master_password: &str,
    ) -> Result<PathBuf, Box<dyn std::error::Error>> {
        info!("Creating backup");
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S").to_string();
        let temp_dir = backup_path.join(format!("backup_{}", timestamp));
        fs::create_dir_all(&temp_dir)?;

        let mut backup_files = Vec::new();

        let dump_path = temp_dir.join(db.path.file_name().unwrap());
        db.create_dump(&dump_path, master_password)?;
        backup_files.push(BackupFile {
            name: db.path.file_name().unwrap().to_str().unwrap().to_string(),
            data: fs::read(dump_path)?,
        });

        for file_name in &["config.toml", ".salt"] {
            backup_files.push(BackupFile {
                name: file_name.to_string(),
                data: fs::read(config_dir.join(file_name))?,
            });
        }

        info!("Compress backup files");
        let compressor = BackupCompressor::new();
        let compressed = compressor.compress(&backup_files)?;

        info!("Encrypt backup data");
        let encrypted = self
            .encryption
            .encrypt(&STANDARD.encode(&compressed))
            .unwrap();

        let final_backup_path = backup_path.join(format!("backup_{}.pmbackup", timestamp));
        fs::write(&final_backup_path, encrypted)?;

        fs::remove_dir_all(temp_dir)?;

        info!("Backup created successfully");
        Ok(final_backup_path)
    }

    /// Restore a backup of the database and configuration files
    ///
    /// # Arguments
    ///
    /// * `backup_file` - The path to the backup file
    /// * `config_dir` - The directory containing the configuration files
    /// * `db` - The database to restore
    /// * `master_password` - The master password to decrypt the backup
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the backup is restored successfully
    ///
    /// # Errors
    ///
    /// Returns an error if the backup restoration fails
    pub fn restore_backup(
        &self,
        backup_file: &Path,
        config_dir: &Path,
        db: &Database,
        master_password: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!("Starting backup restoration from {:?}", backup_file);

        let encrypted_data = fs::read(backup_file)?;
        let decrypted = self
            .encryption
            .decrypt(&encrypted_data)
            .map_err(|_| "Failed to decrypt backup")?;

        let decoded = STANDARD
            .decode(decrypted.as_bytes())
            .map_err(|_| "Failed to decode backup data")?;

        let compressor = BackupCompressor::new();
        let files = compressor.decompress(&decoded)?;

        let temp_dir = backup_file.parent().unwrap().join("restore_temp");
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir)?;
        }
        fs::create_dir_all(&temp_dir)?;

        let has_db = files
            .iter()
            .any(|f| f.name == db.path.file_name().unwrap().to_str().unwrap());
        let has_config = files.iter().any(|f| f.name == "config.toml");
        let has_salt = files.iter().any(|f| f.name == ".salt");

        if !has_db || !has_config || !has_salt {
            return Err("Backup is missing required files".into());
        }

        for file in &files {
            fs::write(temp_dir.join(&file.name), &file.data)?;
        }

        let mut has_backup = false;
        if config_dir.join("config.toml").exists() && config_dir.join(".salt").exists() {
            has_backup = true;
            fs::copy(
                config_dir.join("config.toml"),
                config_dir.join("config.toml.backup"),
            )?;
            fs::copy(config_dir.join(".salt"), config_dir.join(".salt.backup"))?;
        }

        let result: Result<(), Box<dyn std::error::Error>> = (|| {
            fs::copy(temp_dir.join("config.toml"), config_dir.join("config.toml"))?;
            fs::copy(temp_dir.join(".salt"), config_dir.join(".salt"))?;

            db.restore_from_dump(
                &temp_dir.join(db.path.file_name().unwrap()),
                master_password,
            )?;

            Ok(())
        })();

        if result.is_err() && has_backup {
            fs::copy(
                config_dir.join("config.toml.backup"),
                config_dir.join("config.toml"),
            )?;
            fs::copy(config_dir.join(".salt.backup"), config_dir.join(".salt"))?;
        }

        if has_backup {
            let _ = fs::remove_file(config_dir.join("config.toml.backup"));
            let _ = fs::remove_file(config_dir.join(".salt.backup"));
        }
        fs::remove_dir_all(&temp_dir)?;

        result
    }

    /// Automatically create a backup and remove old backups if the maximum number of backups is reached
    ///
    /// # Arguments
    ///
    /// * `db` - The database to backup
    /// * `backup_path` - The directory to store the backup
    /// * `config_dir` - The directory containing the configuration files
    /// * `master_password` - The master password to encrypt the backup
    /// * `max_backups` - The maximum number of backups to keep
    ///
    /// # Returns
    ///
    /// The path to the created backup file
    ///
    /// # Errors
    ///
    /// Returns an error if the backup creation fails
    pub fn auto_backup(
        &self,
        db: &Database,
        backup_path: &Path,
        config_dir: &Path,
        master_password: &str,
        max_backups: usize,
    ) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let new_backup = self.create_backup(db, backup_path, config_dir, master_password)?;
        let mut backups: Vec<_> = fs::read_dir(backup_path)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry
                    .path()
                    .extension()
                    .map_or(false, |ext| ext == "pmbackup")
            })
            .map(|entry| {
                let metadata = entry.metadata().unwrap();
                (entry.path(), metadata.created().unwrap())
            })
            .collect();

        backups.sort_by(|a, b| a.1.cmp(&b.1));

        while backups.len() > max_backups {
            if let Some((path, _)) = backups.first() {
                fs::remove_file(path)?;
                backups.remove(0);
            }
        }

        Ok(new_backup)
    }

    /// Export all password entries to a CSV file
    ///
    /// # Arguments
    ///
    /// * `db` - The database to export
    /// * `path` - The directory to store the CSV file
    ///
    /// # Returns
    ///
    /// The path to the created CSV file
    ///
    /// # Errors
    ///
    /// Returns an error if the export fails
    pub fn export_csv(
        &self,
        db: &Database,
        path: &Path,
    ) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S").to_string();
        let export_path = path.join(format!("password_export_{}.csv", timestamp));

        let entries = db.read_all::<PasswordEntry>()?;

        let mut writer = csv::Writer::from_path(&export_path)?;
        writer.write_record(&["Service", "Username", "Password", "URL", "Notes"])?;

        for entry in entries {
            let password = STANDARD.decode(&entry.password.as_bytes())?;
            let decrypted_pass = self.encryption.decrypt(&password).unwrap();

            writer.write_record(&[
                entry.service,
                entry.username,
                decrypted_pass,
                entry.url,
                entry.notes,
            ])?;
        }

        writer.flush()?;

        Ok(export_path)
    }

    /// Check if a CSV file is valid and safe to import
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path to the CSV file
    ///
    /// # Returns
    ///
    /// Returns `true` if the file is valid and safe to import, `false` otherwise
    ///
    /// # Errors
    ///
    /// Returns an error if the file is invalid or unsafe
    fn is_valid_csv(&self, file_path: &Path) -> Result<bool, Box<dyn std::error::Error>> {
        let metadata = fs::metadata(file_path)?;

        if !metadata.is_file() || metadata.is_symlink() {
            info!("File is not a regular file or is a symlink");
            return Ok(false);
        }

        let file_size = metadata.len();
        if file_size > 10_000_000 {
            info!("File exceeds size limit");
            return Ok(false);
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if metadata.permissions().mode() & 0o111 != 0 {
                info!("File has execute permissions");
                return Ok(false);
            }
        }

        #[cfg(windows)]
        {
            use std::os::windows::fs::MetadataExt;
            let attributes = metadata.file_attributes();
            if attributes & 0x2 != 0 || attributes & 0x4 != 0 {
                info!("File is hidden or system file");
                return Ok(false);
            }
        }

        let mut file = File::open(file_path)?;
        let mut buffer = [0u8; 1024];
        let bytes_read = file.read(&mut buffer)?;

        let stated_size = file.metadata()?.len();
        if stated_size != file_size {
            info!("File size mismatch");
            return Ok(false);
        }

        if buffer[..bytes_read].contains(&0) {
            info!("File contains null bytes");
            return Ok(false);
        }

        if String::from_utf8(buffer[..bytes_read].to_vec()).is_err() {
            info!("File contains invalid UTF-8");
            return Ok(false);
        }

        Ok(true)
    }

    /// Import password entries from a CSV file
    ///
    /// # Arguments
    ///
    /// * `db` - The database to import the entries into
    /// * `file_path` - The path to the CSV file
    ///
    /// # Returns
    ///
    /// The result of the import operation
    ///
    /// # Errors
    ///
    /// Returns an error if the import fails
    pub fn import_csv(
        &self,
        db: &Database,
        file_path: &Path,
    ) -> Result<ImportResult, Box<dyn std::error::Error>> {
        if !self.is_valid_csv(file_path)? {
            return Err("Invalid or potentially unsafe CSV file".into());
        }

        let mut imported = 0;
        let mut skipped = 0;
        let mut errors = Vec::new();

        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b';')
            .has_headers(false)
            .flexible(true)
            .trim(csv::Trim::All)
            .from_path(file_path)?;

        let mut existing_entries = db.read_all::<PasswordEntry>()?;

        for (index, result) in rdr.records().enumerate() {
            let record = match result {
                Ok(rec) => {
                    println!("Record content: {:?}", rec);
                    rec
                }
                Err(e) => {
                    println!("Error: {}", e);
                    errors.push(format!("Error reading line {}: {}", index + 1, e));
                    continue;
                }
            };

            if record.len() != 5 {
                errors.push(format!("Line {} has invalid number of fields", index + 1));
                continue;
            }

            let is_duplicate = existing_entries
                .iter()
                .any(|entry| entry.service == record[0] && entry.username == record[1]);

            if is_duplicate {
                skipped += 1;
                continue;
            }

            let encrypted = self.encryption.encrypt(&record[2]).unwrap();
            let encoded = STANDARD.encode(encrypted);

            let entry = PasswordEntry::new(
                1,
                record[0].to_string(),
                record[1].to_string(),
                encoded,
                record[3].to_string(),
                record[4].to_string(),
            );

            match db.create(&entry) {
                Ok(_) => {
                    imported += 1;
                    existing_entries = db.read_all::<PasswordEntry>()?;
                }
                Err(e) => {
                    errors.push(format!("Error importing entry: {}", e));
                }
            }
        }

        Ok(ImportResult {
            imported,
            skipped,
            errors,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{database::User, Auth};
    use csv::StringRecord;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    fn setup_test_env() -> (TempDir, Database, PathBuf, PathBuf, Encryption) {
        let temp = TempDir::new().unwrap();
        let config_dir = temp.path().join("config");
        let backup_dir = temp.path().join("backups");

        fs::create_dir_all(&config_dir).unwrap();
        fs::create_dir_all(&backup_dir).unwrap();

        let salt = [0u8; 16];
        fs::write(config_dir.join(".salt"), &salt).unwrap();

        fs::write(config_dir.join("config.toml"), "test config").unwrap();

        let db = Database::new(config_dir.join("test.db"), "test_password", &salt).unwrap();
        let encryption = Encryption::new("test_password", &salt);

        (temp, db, config_dir, backup_dir, encryption)
    }

    #[test]
    fn test_create_backup() {
        let (_temp, db, config_dir, backup_dir, encryption) = setup_test_env();

        let auth = Auth::new(&db);
        auth.register("testuser", "testpass").unwrap();

        let backup_manager = BackupManager::new(encryption);
        let backup_path = backup_manager
            .create_backup(&db, &backup_dir, &config_dir, "test_password")
            .unwrap();

        assert!(backup_path.exists());
        assert_eq!(backup_path.extension().unwrap(), "pmbackup");

        let temp_backups = fs::read_dir(&backup_dir)
            .unwrap()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().is_dir())
            .count();
        assert_eq!(temp_backups, 0);
    }

    #[test]
    fn test_backup_restore() {
        let (_temp, db, config_dir, backup_dir, encryption) = setup_test_env();

        let auth = Auth::new(&db);
        auth.register("testuser", "testpass").unwrap();

        let backup_manager = BackupManager::new(encryption);
        let backup_path = backup_manager
            .create_backup(&db, &backup_dir, &config_dir, "test_password")
            .unwrap();

        fs::remove_file(&db.path).unwrap();
        fs::remove_file(config_dir.join("config.toml")).unwrap();
        fs::remove_file(config_dir.join(".salt")).unwrap();

        fs::create_dir_all(&config_dir).unwrap();

        let new_db = Database::new(db.path.clone(), "test_password", &[0u8; 16]).unwrap();

        backup_manager
            .restore_backup(&backup_path, &config_dir, &new_db, "test_password")
            .unwrap();

        assert!(config_dir.join("config.toml").exists());
        assert!(config_dir.join(".salt").exists());

        let users = new_db.read_all::<User>().unwrap();
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].username, "testuser");
    }

    #[test]
    fn test_auto_backup() {
        let (_temp, db, config_dir, backup_dir, encryption) = setup_test_env();

        let auth = Auth::new(&db);
        auth.register("testuser", "testpass").unwrap();

        let backup_manager = BackupManager::new(encryption);
        let max_backups = 2;

        let backup1 = backup_manager
            .auto_backup(&db, &backup_dir, &config_dir, "test_password", max_backups)
            .unwrap();

        std::thread::sleep(std::time::Duration::from_secs(1));

        let backup2 = backup_manager
            .auto_backup(&db, &backup_dir, &config_dir, "test_password", max_backups)
            .unwrap();

        std::thread::sleep(std::time::Duration::from_secs(1));

        let backup3 = backup_manager
            .auto_backup(&db, &backup_dir, &config_dir, "test_password", max_backups)
            .unwrap();

        let existing_backups: Vec<_> = fs::read_dir(&backup_dir)
            .unwrap()
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry
                    .path()
                    .extension()
                    .map_or(false, |ext| ext == "pmbackup")
            })
            .collect();

        assert_eq!(existing_backups.len(), max_backups);
        assert!(!backup1.exists());
        assert!(backup2.exists());
        assert!(backup3.exists());

        let backup2_time = fs::metadata(&backup2).unwrap().created().unwrap();
        let backup3_time = fs::metadata(&backup3).unwrap().created().unwrap();
        assert!(backup3_time > backup2_time);
    }

    #[test]
    fn test_export_csv() {
        let (_temp, db, _config_dir, backup_dir, encryption) = setup_test_env();

        let auth = Auth::new(&db);
        auth.register("testuser", "testpass").unwrap();

        let entries = vec![
            (
                "Service1",
                "user1",
                "pass1",
                "https://service1.com",
                "note1",
            ),
            (
                "Service2",
                "user2",
                "pass2",
                "https://service2.com",
                "note2",
            ),
        ];

        for (service, username, password, url, notes) in entries.iter() {
            let encrypted = encryption.encrypt(password).unwrap();
            let encoded = STANDARD.encode(encrypted);

            let entry = PasswordEntry::new(
                1,
                service.to_string(),
                username.to_string(),
                encoded,
                url.to_string(),
                notes.to_string(),
            );
            db.create(&entry).unwrap();
        }

        let backup_manager = BackupManager::new(encryption);
        let export_path = backup_manager
            .export_csv(&db, backup_dir.as_path())
            .unwrap();

        let mut rdr = csv::Reader::from_path(export_path).unwrap();
        let records: Vec<StringRecord> = rdr.records().map(|r| r.unwrap()).collect();

        let headers = rdr.headers().unwrap();
        let expected_headers = vec!["Service", "Username", "Password", "URL", "Notes"];
        assert_eq!(headers.iter().collect::<Vec<_>>(), expected_headers);

        assert_eq!(records.len(), entries.len());

        for (i, record) in records.iter().enumerate() {
            let (service, username, password, url, notes) = entries[i];
            assert_eq!(record[0], service.to_string());
            assert_eq!(record[1], username.to_string());
            assert_eq!(record[2], password.to_string());
            assert_eq!(record[3], url.to_string());
            assert_eq!(record[4], notes.to_string());
        }
    }

    #[test]
    fn test_valid_csv() {
        let (_temp, _db, _config_dir, backup_dir, encryption) = setup_test_env();
        let backup_manager = BackupManager::new(encryption);

        let test_file = backup_dir.join("test.csv");
        let mut file = File::create(&test_file).unwrap();
        file.write_all(b"service;username;password;url;notes\n")
            .unwrap();
        file.write_all(b"test;user;pass;http://test.com;note\n")
            .unwrap();

        assert!(backup_manager.is_valid_csv(&test_file).unwrap());
    }

    #[test]
    fn test_invalid_csv() {
        let (_temp, _db, _config_dir, backup_dir, encryption) = setup_test_env();
        let backup_manager = BackupManager::new(encryption);

        let test_file = backup_dir.join("test_null.csv");
        let mut file = File::create(&test_file).unwrap();
        file.write_all(b"test\0test").unwrap();
        assert!(!backup_manager.is_valid_csv(&test_file).unwrap());

        let test_file = backup_dir.join("test_large.csv");
        let mut file = File::create(&test_file).unwrap();
        for _ in 0..1_000_000 {
            file.write_all(b"test;test;test;test;test\n").unwrap();
        }
        assert!(!backup_manager.is_valid_csv(&test_file).unwrap());
    }

    #[test]
    fn test_invalid_csv_extended() {
        let (_temp, _db, _config_dir, backup_dir, encryption) = setup_test_env();
        let backup_manager = BackupManager::new(encryption);

        let test_file = backup_dir.join("test_invalid_utf8.csv");
        let mut file = File::create(&test_file).unwrap();
        file.write_all(&[0xFF, 0xFF, 0xFF]).unwrap();
        assert!(!backup_manager.is_valid_csv(&test_file).unwrap());

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let test_file = backup_dir.join("test_executable.csv");
            let mut file = File::create(&test_file).unwrap();
            file.write_all(b"test;test").unwrap();
            let mut perms = file.metadata().unwrap().permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&test_file, perms).unwrap();
            assert!(!backup_manager.is_valid_csv(&test_file).unwrap());
        }
    }

    #[test]
    fn test_import_csv() {
        let (_temp, db, _config_dir, backup_dir, encryption) = setup_test_env();
        let auth = Auth::new(&db);
        auth.register("testuser", "testpass").unwrap();

        let users = db.read_all::<User>().unwrap();
        assert_eq!(users.len(), 1);
        let backup_manager = BackupManager::new(encryption);

        let test_file = backup_dir.join("import_test.csv");
        let mut file = File::create(&test_file).unwrap();

        file.write_all(b"Service1;user1;pass1;https://service1.com;note1\n")
            .unwrap();
        file.write_all(b"Service2;user2;pass2;https://service2.com;note2\n")
            .unwrap();

        file.write_all(b"Service1;user1;pass3;https://service3.com;note3\n")
            .unwrap();

        file.write_all(b"Service3;user3;pass3;https://service3.com\n")
            .unwrap();

        let result = backup_manager.import_csv(&db, &test_file).unwrap();

        assert_eq!(result.imported, 2);
        assert_eq!(result.skipped, 1);
        assert_eq!(result.errors.len(), 1);

        let entries = db.read_all::<PasswordEntry>().unwrap();
        assert_eq!(entries.len(), 2);

        let first = entries.iter().find(|e| e.service == "Service1").unwrap();
        assert_eq!(first.username, "user1");
        assert_eq!(first.url, "https://service1.com");
        assert_eq!(first.notes, "note1");

        let second = entries.iter().find(|e| e.service == "Service2").unwrap();
        assert_eq!(second.username, "user2");
        assert_eq!(second.url, "https://service2.com");
        assert_eq!(second.notes, "note2");
    }
}
