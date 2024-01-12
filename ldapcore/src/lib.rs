use std::{fs, io, path::Path};

use ldap3::{Ldap, LdapConnAsync, LdapError};
use serde::{de::value::Error, Deserialize, Serialize};
#[derive(Debug)]
pub enum LibError {
    IoError(io::Error),
    InvalidConfig(toml::de::Error),
    Ldap(LdapError),
}
impl From<io::Error> for LibError {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<toml::de::Error> for LibError {
    fn from(value: toml::de::Error) -> Self {
        Self::InvalidConfig(value)
    }
}
impl From<ldap3::LdapError> for LibError {
    fn from(err: ldap3::LdapError) -> LibError {
        LibError::Ldap(err)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "TN")]
    pub tn: Vec<Tn>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tn {
    pub namme: String,
    #[serde(rename = "CON")]
    pub con: Con,
    #[serde(rename = "SYNC")]
    pub sync: Sync,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Con {
    pub binddn: String,
    pub bindpw: String,
    pub host: String,
    pub base: String,
    pub tls_verify: bool,
    pub starttls: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sync {
    pub filter: String,
    pub attr: String,
    pub regex: String,
    pub replacewith: String,
    pub showall: bool,
    pub checkmode: bool,
}

pub fn parsconf(filename: &str) -> Result<Root, LibError> {
    let config_path = Path::new(filename);
    if config_path.exists() {
        // The `?` operator tells Rust, if the value is an error, return that error.
        // You can also use the `?` operator on the Option enum.

        let content = fs::read_to_string(config_path)?;
        let config = toml::from_str(&content)?;

        return Ok(config);
    }

    // The config file does not exist, so we must initialize it with the default values.

    let config = Root::default();
    let toml = toml::to_string(&config).unwrap();

    fs::write(config_path, toml)?;

    Ok(config)
}
//hostname: &str, tls_verify: bool, starttls: bool
pub async fn createcon(condata: Con) -> Result<Ldap, LibError> {
    let settings = ldap3::LdapConnSettings::new();
    let dur = core::time::Duration::from_secs(3);
    let settings = settings.set_conn_timeout(dur);
    let settings = settings.set_no_tls_verify(condata.tls_verify);
    let settings = settings.set_starttls(condata.starttls);

    let (conn, mut ldap) = LdapConnAsync::with_settings(settings, &condata.host).await?;
    ldap3::drive!(conn);
    Ok(ldap)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let filename: &str = "Config.toml";
        let result = parsconf(filename);
        println!("{:?}", &result);
        //result.tn.
        //assert_eq!(result, 4);
    }
}
