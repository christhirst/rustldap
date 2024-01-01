use std::collections::HashSet;

use crate::{confload, reg, CliError};
use ldap3::{LdapConn, LdapResult, ResultEntry, Scope, SearchEntry};
use reg::findReplace;

pub fn ldapsearch(
    ldapcon: &mut LdapConn,
    base: &str,
    filter: &str,
) -> Result<Vec<ResultEntry>, CliError> {
    println!("Search");
    let (rs, _res) = ldapcon
        .search(
            //uid=billy,dc=example,dc=org
            base,
            Scope::Subtree,
            filter,
            vec!["*".to_string()],
        )?
        .success()?;

    Ok(rs)
}

pub fn ldapfindreplace(ldapcon: &mut LdapConn) -> Result<(), CliError> {
    let conf = confload("Config.toml")?;
    let mut ldap: LdapConn = LdapConn::new(conf.host.as_str())?;
    let rb = ldap.simple_bind(&conf.binddn, &conf.bindpw)?;

    let result = if rb.rc == 0 { "works" } else { "failed" };
    println!("Bind status: {}", result);

    let rs = ldapsearch(&mut ldap, &conf.base, &conf.filter)?;

    println!("{:?} {:?}", "pre", "after");
    for entry in rs {
        let e = SearchEntry::construct(entry);
        let attr = e.attrs.get(&conf.attr);
        let dn = e.dn;
        if let Some(v) = attr {
            let att = v.first().unwrap().as_str();
            let newattr = findReplace(att, &conf.regex);

            println!("{:?} {:?}", att, newattr);

            let ii = conf.attr.clone();
            if !conf.checkmode {
                let replace = vec![ldap3::Mod::Replace(ii, HashSet::from([newattr]))];

                let res = ldap.modify(&dn, replace)?;
                println!("{}", res);
            }
        }
    }
    Ok(ldap.unbind()?)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    #[test]
    fn it_works() {
        let mut attr = HashMap::new();
        attr.insert("uid", "value1");
        attr.insert("cn", "value2");
        attr.insert("objectClass", "inetOrgPerson");
        attr.insert("sn", "carp");
        //let conf = ldapsearch();

        //assert_eq!(conf, o);
    }
}
