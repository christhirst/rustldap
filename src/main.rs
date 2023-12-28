mod config;

use ldap3::{LdapConn, Scope, SearchEntry, ResultEntry, LdapError};
//use ldap3::result::Result;
use serde::{Serialize, Deserialize};
use std::io;
use config::*;

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

fn ldapsearch(ldapcon: &mut LdapConn,) -> Result<Vec<ResultEntry>,CliError>{
    println!("Hello, world!");
    let (rs, _res) = ldapcon.search(
        "dc=example,dc=org",
        Scope::Subtree,
        "(&(objectClass=*)(dc=*))",
        vec!["l"]
    )?.success()?;
    


    Ok(rs)

}

fn confload()->Result<AppConfig,CliError> {
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




fn main() -> Result<(),CliError>{    
    let conf =confload()?;
    let mut ldap: LdapConn = LdapConn::new(conf.host.as_str())?;
    let rb =ldap.simple_bind(&conf.binddn, &conf.bindpw)?;    
    println!("Reslutcode: {}",rb.rc);

    let rs = ldapsearch(&mut ldap)?;
    
    for entry in rs {
        println!("Hello, world!");
        println!("{:?}", SearchEntry::construct(entry));
    }
    Ok(ldap.unbind()?)


}
