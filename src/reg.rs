use regex::Regex;

fn findReplace(input: &str, re: &str) -> String {
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
        findReplace(hay, r"^ki");
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
