use regex::Regex;

pub fn findReplace(input: &str, re: &str) -> String {
    let re = Regex::new(re).unwrap();

    let ii = re.replace(input, "");
    println!("{}", ii);
    ii.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let hay = "kihubertmueller@schnipp.de";
        let want = "hubertmueller@schnipp.de";
        let reg = r"^ki";
        let result = findReplace(hay, reg);
        assert_eq!(result, want);
    }
}
