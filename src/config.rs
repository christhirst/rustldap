
use std::{io, path::Path, fs};

use serde::{Serialize, Deserialize};
use  toml::de::Error;


#[derive(Debug)]
pub enum ConfigError {
    IoError(io::Error),
    InvalidConfig(toml::de::Error)
}
// These implementations allow us to use the `?` operator on functions that
// don't necessarily return ConfigError.
impl From<io::Error> for ConfigError {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(value: toml::de::Error) -> Self {
        Self::InvalidConfig(value)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub binddn: String,
    pub bindpw: String,
    pub host: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            binddn: "Banana".to_string(),
            bindpw: "Bananas".to_string(),
            host: "127.0.0.1:389".to_string(),
        }
    }
}
pub fn load_or_initialize() -> Result<AppConfig, ConfigError> {
    let config_path = Path::new("Config.toml");

    if config_path.exists() {
        // The `?` operator tells Rust, if the value is an error, return that error.
        // You can also use the `?` operator on the Option enum.

        let content = fs::read_to_string(config_path)?;
        let config = toml::from_str(&content)?;

        return Ok(config);
    }

    // The config file does not exist, so we must initialize it with the default values.

    let config = AppConfig::default();
    let toml = toml::to_string(&config).unwrap();

    fs::write(config_path, toml)?;
    Ok(config)
}