use chrono::{DateTime, Utc};
use std::{collections::HashMap, error::Error, io, time::SystemTime};
use tracing::warn;

use ldapcore::{createcon, LibError, Tn};

use crate::config::ConfigError;

#[derive(Debug)]
pub struct check {
    pub tn: String,
    pub date: String,
    pub problemtsl: u32,
    pub problemnet: u32,
}

pub async fn checkcons(cons_config: Vec<Tn>) -> Result<HashMap<String, check>, ()> {
    let mut res = HashMap::new();

    for i in cons_config {
        let now = Utc::now().to_rfc3339();

        let mut tn = check {
            tn: i.name.clone(),
            date: now,
            problemtsl: 0,
            problemnet: 0,
        };
        let con = createcon(&i.con).await;
        match con {
            Err(i) => match i {
                LibError::IoError(e) => {
                    let s = e.source().unwrap().to_string().contains("NativeTLS");
                    if s {
                        tn.problemtsl += 1;
                    } else {
                        warn!("Error: {}  1", e);
                    }
                }
                LibError::InvalidConfig(e) => println!("Error: {:?} 23", e),
                LibError::Ldap(e) => {
                    println!(
                        "Error: {:?} - {:?} 434",
                        e.source().unwrap().to_string().contains("certificate"),
                        e
                    )
                }

                _ => todo!(),
            },
            Ok(mut x) => {
                //println!("{:?}", x);
                x.simple_bind(&i.con.host, &i.con.bindpw);
            }
        }

        res.insert(i.name, tn);
    }
    Ok(res)
}

#[cfg(test)]
mod tests {

    use std::vec;

    use ldapcore::parsconf;

    use super::*;
    #[tokio::test]
    async fn config_parse() {
        let filename = "Config2.toml";
        let result = parsconf(filename).unwrap();
        println!("{:?}", "result.err()");
        //println!("{:?}", result.err());
        let conf = checkcons(result.tn).await;
        // println!(conf)

        //findReplace(hay, r"^ki");
        //let result = 2 + 2;
    }
}
