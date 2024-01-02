mod config;
mod ldapcrud;
mod prettytab;
mod reg;

use serde_json::{json, Value};

use axum::{routing::get, Json, Router};
use ldap3::{LdapConn, LdapError};
//use ldap3::result::Result;
use config::*;
use ldapcrud::ldapfindreplace;

use crate::{
    ldapcrud::{get_plan, ldapsearch},
    prettytab::printastab,
};

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

async fn json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

#[tokio::main]
async fn main() -> Result<(), CliError> {
    let app = Router::new().route("/json", get(json));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    let file = "Config.toml";
    let conf = confload(file)?;
    let mut ldapcon: LdapConn = LdapConn::new(conf.host.as_str())?;
    let rb = ldapcon.simple_bind(&conf.binddn, &conf.bindpw)?;
    let result = if rb.rc == 0 { "works" } else { "failed" };
    println!("Bind status: {}", result);

    let conf = confload(file)?;
    //let plan = get_plan(&mut ldapcon, &conf);

    // let res = ldapfindreplace(&mut ldapcon, &conf);
    let rs = ldapsearch(&mut ldapcon, &conf.base, &conf.filter)?;
    let plan = get_plan(&rs, &conf);
    let title = vec!["dn", "attr", "regex", "replace", "Before", "After"];
    let new_vector: Vec<Vec<&str>> = plan
        .iter()
        .map(|inner| inner.iter().map(|s| s.as_str()).collect())
        .collect();
    let _rs = ldapfindreplace(&mut ldapcon, &plan, conf.checkmode)?;
    printastab(title, new_vector);
    ldapcon.unbind()?;
    Ok(())
}
