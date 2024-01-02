use regex::Regex;

pub fn find_replace<'a>(input: &'a str, re: &str, rep: &str) -> String {
    let re = Regex::new(re).unwrap();

    let ii = re.replace(input, rep);
    let replaced = ii.to_string();
    replaced
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let hay = "kihubertmueller@schnipp.de";
        let want = "hubertmueller@schnipp.de";
        let reg = r"^ki";
        let result = find_replace(hay, reg, "");
        assert_eq!(result, want);
    }
}
