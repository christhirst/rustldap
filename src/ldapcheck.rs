use std::{error::Error, io};

use ldapcore::{createcon, LibError, Tn};

pub async fn checkcons(cons_config: Vec<Tn>) {
    for i in cons_config {
        let con = createcon(&i.con).await;
        match con {
            Err(i) => match i {
                LibError::IoError(e) => println!(
                    "Error: {:?}  1",
                    e.source().unwrap().to_string().contains("NativeTLS")
                ),
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
                println!("{:?}", x);
                x.simple_bind(&i.con.host, &i.con.bindpw);
            }
        }
    }
}

#[cfg(test)]
mod tests {
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
