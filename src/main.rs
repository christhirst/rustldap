mod config;
mod ldapcrud;
mod prettytab;
mod reg;

use ldap3::{LdapConn, LdapError, ResultEntry, Scope, SearchEntry};
//use ldap3::result::Result;
use config::*;
use ldapcrud::ldapfindreplace;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, io};

//use crate::ldapcrud::ldapfindreplace;

#[derive(Debug)]
pub enum CliError {
    EntityNotFound { entity: &'static str, id: i64 },
    Ldap(LdapError),
    ConfigError(ConfigError),

    FailedToCreatePool(String),
}

impl From<ldap3::LdapError> for CliError {
    fn from(err: ldap3::LdapError) -> CliError {
        CliError::Ldap(err)
    }
}

impl From<config::ConfigError> for CliError {
    fn from(err: config::ConfigError) -> CliError {
        CliError::ConfigError(err)
    }
}

fn confload(file: &str) -> Result<AppConfig, CliError> {
    let config: AppConfig = match load_or_initialize(file) {
        Ok(v) => v,
        Err(err) => {
            /* match err {
                ConfigError::IoError(err) => {
                    eprintln!("An error occurred while loading the config: {err}");
                }
                ConfigError::InvalidConfig(err) => {
                    eprintln!("An error occurred while parsing the config:");
                    eprintln!("{err}");
                }
            } */
            return Err(err.into());
        }
    };

    return Ok(config);
    //println!("{:?}", config);
}

fn main() -> Result<(), CliError> {
    let file = "Config.toml";
    let conf = confload(file)?;
    let mut ldap: LdapConn = LdapConn::new(conf.host.as_str())?;
    let rb = ldap.simple_bind(&conf.binddn, &conf.bindpw)?;
    println!("Reslutcode: {}", rb.rc);

    let res = ldapfindreplace(&mut ldap);
    res
}
