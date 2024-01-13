use std::io;

use ldapcore::{createcon, LibError, Tn};

pub async fn checkcons(cons_config: Vec<Tn>) {
    for i in cons_config {
        let con = createcon(i.con).await;
        match con {
            Err(i) => match i {
                LibError::IoError(e) => println!("Error: {:?}  1", e),
                LibError::InvalidConfig(e) => println!("Error: {:?} 23", e),
                LibError::Ldap(e) => println!("Error: {:?} 434", e),

                _ => todo!(),
            },
            Ok(x) => println!("{:?}", x),
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
