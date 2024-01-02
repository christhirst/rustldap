use std::{collections::HashSet, vec};

use crate::{config::AppConfig, confload, reg, CliError};
use ldap3::{LdapConn, LdapResult, ResultEntry, Scope, SearchEntry};
use reg::find_Replace;

pub fn ldapsearch(
    ldapcon: &mut LdapConn,
    base: &str,
    filter: &str,
) -> Result<Vec<ResultEntry>, CliError> {
    let (rs, _res) = ldapcon
        .search(base, Scope::Subtree, filter, vec!["*".to_string()])?
        .success()?;
    Ok(rs)
}

pub fn vec_add_data<'a, 'b>(
    data: &'b mut Vec<Vec<String>>,
    dn: String,
    attr: String,
    regex: String,
    replace: String,
    before: String,
    after: String,
) {
    //-> &'a mut Vec<&'a str>
    let row: Vec<String> = vec![dn, attr, regex, replace, before, after];
    data.push(row);
}

/* pub fn get_plan(
    ldapcon: &mut LdapConn,
    conf: &AppConfig,
    dn: &str,
    attr: &str,
    newattr: &str,
) -> Vec<Vec<&str>> {
    let mut tab: Vec<Vec<&str>> = Vec::new();
    vec_add_data(
        &mut tab,
        dn,
        &conf.attr,
        &conf.regex,
        &conf.replacewith,
        attr,
        newattr,
    );

    todo!()
} */

pub fn get_plan<'c>(entries: &'c Vec<ResultEntry>, conf: &'c AppConfig) -> Vec<Vec<String>> {
    let mut tab: Vec<Vec<String>> = Vec::new();
    for bin_entry in entries.clone() {
        let entry: SearchEntry = SearchEntry::construct(bin_entry).clone();
        if let Some(attr) = entry.attrs.get(&conf.attr).and_then(|v| v.first()) {
            let newattr = find_Replace(attr, &conf.regex, &conf.replacewith);
            vec_add_data(
                &mut tab,
                entry.dn,
                conf.attr.to_owned(),
                conf.regex.to_owned(),
                conf.replacewith.to_owned(),
                attr.to_string(),
                newattr.to_owned(),
            )
        }
    }
    tab
    //todo!()
}

pub fn ldapfindreplace(ldapcon: &mut LdapConn, conf: &AppConfig) -> Result<(), CliError> {
    //let conf = confload(conf)?;
    //let mut ldap: LdapConn = LdapConn::new(conf.host.as_str())?;

    let rs = ldapsearch(ldapcon, &conf.base, &conf.filter)?;

    for entry in rs {
        //convert to entry
        let e: SearchEntry = SearchEntry::construct(entry);
        //getting the attribute value from entry
        let mut tab: Vec<Vec<&str>> = Vec::new();
        if let Some(attr) = e.attrs.get(&conf.attr).and_then(|v| v.first()) {
            let newattr = find_Replace(attr, &conf.regex, &conf.replacewith);
            /* vec_add_data(
                &mut tab,
                &e.dn,
                &conf.attr,
                &conf.regex,
                &conf.replacewith,
                attr,
                newattr,
            ); */
            /* if !conf.checkmode {
                let res = ldapcon.modify(
                    &e.dn,
                    vec![ldap3::Mod::Replace(
                        conf.attr.clone(),
                        HashSet::from([newattr]),
                    )],
                )?;
                println!("Modify{}", res);
            } */
        };
    }
    //Ok()
    todo!()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    #[test]
    fn it_works() {
        let mut tab: Vec<Vec<String>> = Vec::new();

        let dn = "dn".to_owned();
        let attr = "attr".to_owned();
        let regex = "regex".to_owned();
        let replace = "replace".to_owned();
        let before = "before".to_owned();
        let after = "after".to_string();
        let dn1 = "dn".to_owned();
        let attr1 = "attr".to_owned();
        let regex1 = "regex".to_owned();
        let replace1 = "replace".to_owned();
        let before1 = "before".to_owned();
        let after1 = "after".to_string();
        vec_add_data(&mut tab, dn, attr, regex, replace, before, after);
        vec_add_data(&mut tab, dn1, attr1, regex1, replace1, before1, after1);
        println!("{:?}", tab);
        //assert_eq!(conf, o);
    }
    #[test]
    #[ignore]
    fn test_get_plan() {
        let file = "Config.toml";
        let conf = confload(file).unwrap();
        let mut ldapcon: LdapConn = LdapConn::new(conf.host.as_str()).unwrap();
        ldapcon.simple_bind(&conf.binddn, &conf.bindpw).unwrap();
        let rs = ldapsearch(&mut ldapcon, &conf.base, &conf.filter).unwrap();

        let conf = AppConfig::default();
        let plan = get_plan(&rs, &conf);
        println!("{:?}", plan)
        //assert_eq!(conf, o);
    }
}
