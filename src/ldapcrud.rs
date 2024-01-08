use std::{collections::HashSet, vec};

use crate::{config::AppConfig, reg, CliError};

use ldap3::{Ldap, LdapConn, LdapResult, ResultEntry, Scope, SearchEntry};

use reg::find_replace;

pub async fn ldapsearch(
    ldapcon: &mut Ldap,
    base: &str,
    filter: &str,
) -> Result<Vec<ResultEntry>, CliError> {
    let (rs, _res) = ldapcon
        .search(base, Scope::Subtree, filter, vec!["*".to_string()])
        .await?
        .success()?;

    //;
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

pub fn get_plan<'c>(entries: &'c Vec<ResultEntry>, conf: &'c AppConfig) -> Vec<Vec<String>> {
    let mut tab: Vec<Vec<String>> = Vec::new();
    for bin_entry in entries.clone() {
        let entry: SearchEntry = SearchEntry::construct(bin_entry).clone();
        if let Some(attr) = entry.attrs.get(&conf.attr).and_then(|v| v.first()) {
            let newattr = find_replace(attr, &conf.regex, &conf.replacewith);
            if newattr != attr.to_string() {
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
    }
    tab
    //todo!()
}

pub async fn ldapfindreplace(
    ldapcon: &mut Ldap,
    plan: &Vec<Vec<String>>,
    checkmode: bool,
) -> Result<Vec<LdapResult>, CliError> {
    let mut rs_vec: Vec<LdapResult> = Vec::new();
    if !checkmode {
        for entry in plan {
            let dn = entry[0].as_str();
            let attr = entry[1].as_str();
            let newattr = entry[5].as_str();
            let res = ldapcon
                .modify(
                    dn,
                    vec![ldap3::Mod::Replace(attr, HashSet::from([newattr]))],
                )
                .await?
                .success()?;
            rs_vec.push(res);
        }
    }
    Ok(rs_vec)
}

#[cfg(test)]
mod tests {

    use ldap3::LdapConnAsync;

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

    #[ignore]
    #[tokio::test]
    async fn test_get_plan() -> Result<(), CliError> {
        use crate::confload;
        let file = "Config.toml";
        let conf = confload(file).unwrap();
        //let mut ldapcon: LdapConn = LdapConn::new(conf.host.as_str()).unwrap();
        let (conn, mut ldapcon) = LdapConnAsync::new(conf.host.as_str()).await?;
        ldap3::drive!(conn);

        let b = ldapcon
            .simple_bind(&conf.binddn, &conf.bindpw)
            .await?
            .success()?;
        let rs = ldapsearch(&mut ldapcon, &conf.base, &conf.filter).await?;
        //let conf = AppConfig::default();
        let plan = get_plan(&rs, &conf);
        println!("{:?}", plan);

        //assert_eq!(conf, o);
        Ok(())
    }
}
