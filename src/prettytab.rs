use prettytable::{format, row, Cell, Row, Table};

pub fn printastab() {
    let mut table = Table::new();
    let format = format::FormatBuilder::new()
        .column_separator('|')
        .borders('|')
        .separators(
            &[format::LinePosition::Top, format::LinePosition::Bottom],
            format::LineSeparator::new('-', '+', '+', '+'),
        )
        .padding(1, 1)
        .build();
    table.set_format(format);

    table.set_titles(row!["Title 1", "Title 2"]);
    table.add_row(row!["Value 1", "Value 2"]);
    table.add_row(row!["Value three", "Value four"]);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let hay = "kihubertmueller@schnipp.de";
        let want = "hubertmueller@schnipp.de";
        let reg = r"^ki";
        let result = printastab();
        //assert_eq!(result, want);
    }
}
