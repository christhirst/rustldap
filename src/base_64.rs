use base64::{
    self, alphabet,
    engine::{
        self,
        general_purpose::{self},
    },
    Engine,
};

pub fn dec(message: &str) -> String {
    // Import the base64 crate

    // Define your string

    // Encode your string
    let b64 = general_purpose::STANDARD.encode(message);

    println!("{:?}", "wwwwww");
    let bytes = general_purpose::STANDARD.decode(b64.clone()).unwrap();
    println!("{:?}", bytes);
    let s = String::from_utf8(bytes).unwrap();
    println!("{:?}", s);
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn config_parse() {
        let message = "Hello@world.de";
        let conf = dec(message);
        //findReplace(hay, r"^ki");
        //let result = 2 + 2;
        assert_eq!(conf, message);
    }
}
