mod config;
mod ldapfind;
mod reg;

use ldap3::{LdapConn, LdapError, ResultEntry, Scope, SearchEntry};
//use ldap3::result::Result;
use config::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, io};

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

fn confload() -> Result<AppConfig, CliError> {
    let config: AppConfig = match load_or_initialize() {
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
    let conf = confload()?;
    let mut ldap: LdapConn = LdapConn::new(conf.host.as_str())?;
    let rb = ldap.simple_bind(&conf.binddn, &conf.bindpw)?;
    println!("Reslutcode: {}", rb.rc);

    let rs = ldapfind::ldapsearch(&mut ldap, "", &conf.filter)?;

    let attrs = vec![
        ("uid", HashSet::from(["billy"])),
        ("cn", HashSet::from(["billy"])),
        ("objectClass", HashSet::from(["top", "inetOrgPerson"])),
        ("sn", HashSet::from(["3"])),
        //("cn", HashSet::from(["billy"])),
    ];

    let res = ldap.add("uid=billy,dc=example,dc=org", attrs)?;

    let replace = vec![ldap3::Mod::Replace(
        "sn".to_string(),
        HashSet::from(["billy".to_string()]),
    )];

    let res = ldap.modify("uid=billy,dc=example,dc=org", replace)?;
    println!("{}", res);

    for entry in rs {
        println!("Hello, world!");
        println!("{:?}", SearchEntry::construct(entry));
    }
    Ok(ldap.unbind()?)
}
