use std::{fs, io, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum ConfigError {
    IoError(io::Error),
    InvalidConfig(toml::de::Error),
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct AppConfig {
    pub binddn: String,
    pub bindpw: String,
    pub host: String,
    pub base: String,
    pub filter: String,
    pub attr: String,

    pub regex: String,
    pub replacewith: String,
    pub showall: bool,
    pub checkmode: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            binddn: "cn=admin,dc=example,dc=org".to_string(),
            bindpw: "admin".to_string(),
            host: "ldap://0.0.0.0:389".to_string(),
            base: "dc=example,dc=org".to_string(),
            filter: "(&(objectClass=*)(cn=*)(!(sn=*billy)))".to_string(),
            attr: "sn".to_string(),
            regex: "^ki".to_string(),
            replacewith: "ka".to_string(),
            showall: true,
            checkmode: true,
        }
    }
}
pub fn load_or_initialize(filename: &str) -> Result<AppConfig, ConfigError> {
    let config_path = Path::new(filename);

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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn config_parse() {
        let filename = "Config.toml";
        let conf = load_or_initialize(filename).unwrap();
        //findReplace(hay, r"^ki");
        //let result = 2 + 2;
        let o = AppConfig::default();
        assert_eq!(conf, o);
    }
}
