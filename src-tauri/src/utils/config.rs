use env_logger::{Builder, WriteStyle};
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::path::PathBuf;

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub logging: LogConfig,
    pub database: DatabaseConfig,
    pub app: AppConfig,
    pub generator: GeneratorConfig,
    pub backup: BackupConfig,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub is_initialized: bool,
    pub auto_logout_duration: u64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LogConfig {
    pub level: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub db_name: String,
    pub db_path: PathBuf,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GeneratorConfig {
    pub default_length: usize,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BackupConfig {
    pub enabled: bool,
    pub interval: BackupInterval,
    pub max_backups: usize,
    pub backup_path: PathBuf,
    pub last_backup: Option<String>,
    pub export_path: PathBuf,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum BackupInterval {
    Daily,
    Weekly,
    Monthly,
}

impl Default for BackupInterval {
    fn default() -> Self {
        Self::Weekly
    }
}

impl Default for Config {
    fn default() -> Self {
        let config_dir = Config::get_config_dir().unwrap();
        let export_path = dirs::document_dir()
            .unwrap()
            .join("karnes-development/karncrypt/exports");
        Config {
            logging: LogConfig {
                level: "info".to_string(),
            },
            database: DatabaseConfig {
                db_name: "pass.db".to_string(),
                db_path: config_dir.clone(),
            },
            app: AppConfig {
                is_initialized: false,
                auto_logout_duration: 10,
            },
            generator: GeneratorConfig { default_length: 16 },
            backup: BackupConfig {
                enabled: false,
                interval: BackupInterval::default(),
                max_backups: 7,
                backup_path: config_dir.join("backups"),
                last_backup: None,
                export_path,
            },
        }
    }
}

impl Config {
    /// Load the configuration from the config file.
    ///
    /// # Returns
    ///
    /// A Result containing the Config struct or an error.
    ///
    /// # Errors
    ///
    /// If the config file cannot be read or written to.
    pub fn load() -> Result<Config, Box<dyn std::error::Error>> {
        let config_dir = Self::get_config_dir()?;
        let config_path = config_dir.join("config.toml");

        if config_path.exists() {
            let config_file = std::fs::read_to_string(config_path)?;
            Ok(toml::from_str(&config_file)?)
        } else {
            Ok(Config::default())
        }
    }

    /// Save the configuration to the config file.
    ///
    /// # Returns
    ///
    /// A Result containing the Config struct or an error.
    ///
    /// # Errors
    ///
    /// If the config file cannot be written to.
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_dir = Self::get_config_dir()?;
        let config_path = config_dir.join("config.toml");
        let config_file = toml::to_string_pretty(&self)?;
        std::fs::write(config_path, config_file)?;

        Ok(())
    }

    /// Get the config directory.
    ///
    /// # Returns
    ///
    /// A Result containing the Config struct or an error.
    ///
    /// # Errors
    ///
    /// If the config file cannot be written to.
    pub fn get_config_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let config_dir = dirs::config_dir().ok_or("Could not find config directory")?;
        let app_dir = config_dir.join("karnes-development/karncrypt");
        if !app_dir.exists() {
            std::fs::create_dir_all(&app_dir)?;
        }
        Ok(app_dir)
    }

    /// Get the log directory.
    ///
    /// If the log directory does not exist, create it.
    ///
    /// # Returns
    ///
    /// A Result containing the log directory or an error.
    ///
    /// # Errors
    ///
    /// If the log directory cannot be created.
    fn get_log_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let config_dir = Self::get_config_dir()?;
        let log_dir = config_dir.join("logs");

        if !log_dir.exists() {
            std::fs::create_dir_all(&log_dir)?;
        }

        Ok(log_dir)
    }

    /// Setup the logger with the configuration from the config file.
    ///
    /// # Returns
    ///
    /// A Result containing the logger or an error.
    ///
    /// # Errors
    ///
    /// If the log file cannot be opened or written to.
    pub fn setup_logger(&self) -> Result<(), Box<dyn std::error::Error>> {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let log_dir = Self::get_log_dir()?;
        let log_file = log_dir.join(format!("karncrypt-{}.log", today));

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file)?;

        let level = match self.logging.level.as_str() {
            "trace" => LevelFilter::Trace,
            "debug" => LevelFilter::Debug,
            "info" => LevelFilter::Info,
            "warn" => LevelFilter::Warn,
            "error" => LevelFilter::Error,
            _ => LevelFilter::Info,
        };

        Builder::new()
            .filter(None, level)
            .write_style(WriteStyle::Always)
            .target(env_logger::Target::Pipe(Box::new(file)))
            .init();

        Ok(())
    }

    /// Get the database directory.
    ///
    /// If the database directory does not exist, create it.
    ///
    /// # Returns
    ///
    /// A Result containing the path to the database file or an error.
    ///
    /// # Errors
    ///
    /// If the database file path cannot be created.
    pub fn get_db_dir(&self) -> Result<&PathBuf, Box<dyn std::error::Error>> {
        let db_dir = &self.database.db_path;

        if !db_dir.exists() {
            std::fs::create_dir_all(&db_dir)?;
        }

        Ok(db_dir)
    }

    /// Get the directory for backups.
    ///
    /// If the backup directory does not exist, create it.
    ///
    /// # Returns
    ///
    /// A Result containing the path to the backup directory or an error.
    ///
    /// # Errors
    ///
    /// If the backup directory path cannot be created.
    pub fn get_backup_dir(&self) -> Result<&PathBuf, Box<dyn std::error::Error>> {
        let backup_dir = &self.backup.backup_path;

        if !backup_dir.exists() {
            std::fs::create_dir_all(&backup_dir)?;
        }

        Ok(backup_dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();

        assert_eq!(config.logging.level, "info");
        assert_eq!(config.database.db_name, "pass.db");
        assert!(!config.app.is_initialized);
        assert_eq!(config.app.auto_logout_duration, 10);
        assert_eq!(config.generator.default_length, 16);
        assert!(!config.backup.enabled);
        assert!(matches!(config.backup.interval, BackupInterval::Weekly));
        assert_eq!(config.backup.max_backups, 7);
        assert!(config.backup.last_backup.is_none());
    }
}
