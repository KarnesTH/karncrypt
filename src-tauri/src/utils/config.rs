use env_logger::{Builder, WriteStyle};
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub logging: LogConfig,
    pub database: DatabaseConfig,
    pub app: AppConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub is_initialized: bool,
    pub db_custom_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogConfig {
    pub level: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub db_name: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            logging: LogConfig {
                level: "info".to_string(),
            },
            database: DatabaseConfig {
                db_name: "pass.db".to_string(),
            },
            app: AppConfig {
                is_initialized: false,
                db_custom_path: None,
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
            let config = Config::default();
            config.save()?;
            Ok(config)
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
        let app_dir = config_dir.join("karnes-development/password-manager");
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
        let log_file = log_dir.join(format!("password-manager-{}.log", today));

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

    /// Get the path to the database file.
    ///
    /// # Returns
    ///
    /// A Result containing the path to the database file or an error.
    ///
    /// # Errors
    ///
    /// If the database file path cannot be created.
    pub fn get_db_path(&self) -> Result<PathBuf, Box<dyn std::error::Error>> {
        if let Some(path) = &self.app.db_custom_path {
            Ok(PathBuf::from(path))
        } else {
            let config_dir = Self::get_config_dir()?;
            Ok(config_dir.join(&self.database.db_name))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config() {
        let config = Config::load().unwrap();
        assert_eq!(config.logging.level, "info");
        assert_eq!(config.database.db_name, "pass.db");
    }

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.logging.level, "info");
        assert_eq!(config.database.db_name, "pass.db");
    }

    #[test]
    fn test_get_config_dir() {
        let config_dir = Config::get_config_dir().unwrap();
        assert!(config_dir.exists());
    }

    #[test]
    fn test_get_log_dir() {
        let log_dir = Config::get_log_dir().unwrap();
        assert!(log_dir.exists());
    }

    #[test]
    fn test_get_db_path() {
        let config = Config::load().unwrap();
        let db_path = config.get_db_path().unwrap();
        assert!(db_path.exists());
    }

    #[test]
    fn test_setup_logger() {
        let config = Config::default();
        let result = config.setup_logger();

        assert!(result.is_ok());
    }
}
