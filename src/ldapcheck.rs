use ldapcore::{createcon, Tn};

pub async fn checkcons(cons_config: Vec<Tn>) {
    for i in cons_config {
        let con = createcon(i.con);
    }
}
