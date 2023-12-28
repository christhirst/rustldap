use ldap3::{LdapConn, ResultEntry, Scope};

use crate::CliError;

pub(crate) fn ldapsearch(
    ldapcon: &mut LdapConn,
    base: &str,
    filter: &str,
) -> Result<Vec<ResultEntry>, CliError> {
    println!("Hello, world!");
    let (rs, _res) = ldapcon
        .search(
            //uid=billy,dc=example,dc=org
            "dc=example,dc=org",
            Scope::Subtree,
            filter,
            vec!["*".to_string()],
        )?
        .success()?;

    Ok(rs)
}
