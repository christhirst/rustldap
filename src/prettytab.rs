use prettytable::{format, row, Cell, Row, Table};

fn rowfromstr(table: &mut Table, data: Vec<Vec<&str>>) {
    for row in data {
        let vec_of_cell: Vec<Cell> = row.into_iter().map(Cell::new).collect();
        table.add_row(Row::new(vec_of_cell));
    }
    table.printstd();
}

pub fn printastab(title: Vec<&str>, data: Vec<Vec<&str>>) {
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
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    table.set_titles(title.into_iter().map(Cell::new).collect());

    rowfromstr(&mut table, data)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let hay = "kihubertmueller@schnipp.de";
        let want = "hubertmueller@schnipp.de";
        let reg = r"^ki";
        let title = vec!["dn", "attr", "regex", "replace", "Before", "After"];
        let data: Vec<Vec<&str>> = vec![vec!["1", "2"], vec!["3", "4"], vec!["6", "q"]];
        let result = printastab(title, data);
        //assert_eq!(result, want);
    }
}
