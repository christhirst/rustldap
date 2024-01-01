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
        //convert to entry
        let e: SearchEntry = SearchEntry::construct(entry);
        //getting the attribute value from entry
        if let Some(attr) = e.attrs.get(&conf.attr).and_then(|v| v.first()) {
            let newattr = findReplace(attr, &conf.regex, &conf.replacewith);
            if !conf.checkmode {
                let res = ldap.modify(
                    &e.dn,
                    vec![ldap3::Mod::Replace(
                        conf.attr.clone(),
                        HashSet::from([newattr]),
                    )],
                )?;
                println!("Modify{}", res);
            }
        };
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
