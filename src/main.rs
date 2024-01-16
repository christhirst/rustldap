mod base_64;
mod config;
mod ldapcheck;
mod ldapcrud;
mod prettytab;
mod reg;
use ldapcore::parsconf;
use std::fs;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

use serde_json::{json, Value};

use axum::{routing::get, Extension, Json, Router};
use ldap3::{Ldap, LdapConnAsync, LdapError};
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

async fn json(
    Extension(conf): Extension<AppConfig>,
    Extension(mut ldap): Extension<Ldap>,
) -> Json<Value> {
    let rs = ldapsearch(&mut ldap, &conf.base, &conf.filter)
        .await
        .unwrap();
    let plan = get_plan(&rs, &conf);
    let _rs: Vec<ldap3::LdapResult> = ldapfindreplace(&mut ldap, &plan, conf.checkmode)
        .await
        .unwrap();
    let title = vec!["dn", "attr", "regex", "replace", "Before", "After"];
    let new_vector: Vec<Vec<&str>> = plan
        .iter()
        .map(|inner| inner.iter().map(|s| s.as_str()).collect())
        .collect();
    printastab(title, new_vector);
    Json(json!({ "data": 42 }))
}

#[tokio::main]
async fn main() -> Result<(), CliError> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let path = ".";
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                println!("{}", entry.file_name().to_string_lossy());
            }
        }
    }

    let file = "Config.toml";
    let conf = confload(file)?;
    let n = ldap3::LdapConnSettings::new();

    let (conn, mut ldap) = LdapConnAsync::with_settings(n, conf.host.as_str()).await?;
    ldap3::drive!(conn);

    let rb = ldap.simple_bind(&conf.binddn, &conf.bindpw).await?;
    let result = if rb.rc == 0 { "works" } else { "failed" };
    println!("Bind status: {}", result);

    let conf = confload(file)?;
    //let plan = get_plan(&mut ldapcon, &conf);

    let rs = ldapsearch(&mut ldap, &conf.base, &conf.filter).await?;
    let plan = get_plan(&rs, &conf);

    let _new_vector: Vec<Vec<&str>> = plan
        .iter()
        .map(|inner| inner.iter().map(|s| s.as_str()).collect())
        .collect();
    let _rs: Vec<ldap3::LdapResult> = ldapfindreplace(&mut ldap, &plan, conf.checkmode).await?;

    let router = Router::new()
        .route("/json", get(json))
        .route("/create", get(json))
        .layer(Extension(ldap))
        .layer(Extension(conf));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
    //ldap.unbind().await?;
    Ok(())
}
