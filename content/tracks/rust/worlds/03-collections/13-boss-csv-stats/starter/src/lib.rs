use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    pub name: String,
    pub category: String,
    pub price: f64,
    pub qty: u32,
}

impl Row {
    /// The money tied up in this line: price * qty. 1.5 * 10 -> 15.0
    pub fn total(&self) -> f64 {
        // TODO: price times qty (qty is a u32, so `self.qty as f64`)
        todo!()
    }
}

/// One line like "pen, office, 1.50, 10" -> Row. Fields are trimmed.
/// Errors: "expected 4 fields, got 2", "bad price 'abc'", "bad qty '-3'".
pub fn parse_line(line: &str) -> Result<Row, String> {
    // TODO: split on ',' and trim into a Vec<&str>; check len() == 4
    // parse fields[2] as f64 and fields[3] as u32, using map_err(...) and `?`
    // build the Row with name and category as Strings
    todo!()
}

/// The whole CSV text. Line 1 is a header and is skipped; blank lines are skipped.
/// A bad line gives Err("line N: <reason>") where N counts from 1 over ALL lines
/// (header = line 1, blank lines count too).
pub fn parse_rows(csv: &str) -> Result<Vec<Row>, String> {
    // TODO: csv.lines().enumerate().skip(1) -> filter out blank lines (trim().is_empty())
    // map each (i, line) to parse_line(line).map_err(|e| format!("line {}: {e}", i + 1))
    // collect() into the Result<Vec<Row>, String>
    todo!()
}

/// Sum of price * qty for each category, keys sorted by name.
pub fn total_by_category(rows: &[Row]) -> BTreeMap<String, f64> {
    // TODO: BTreeMap::new(); for each row: *totals.entry(row.category.clone()).or_insert(0.0) += row.total()
    todo!()
}

/// The `n` rows with the biggest price * qty, biggest first.
pub fn top_n(rows: &[Row], n: usize) -> Vec<&Row> {
    // TODO: collect row references into a Vec, sort_by(|a, b| b.total().total_cmp(&a.total())), take(n)
    todo!()
}

/// The row with the lowest price among rows in `category`. None if there are none.
pub fn cheapest_in<'a>(rows: &'a [Row], category: &str) -> Option<&'a Row> {
    // TODO: filter on r.category == category, then min_by(|a, b| a.price.total_cmp(&b.price))
    todo!()
}

/// A text table, lines joined with "\n" and no trailing newline:
///   header: format!("{:<10} {:<9} {:>8} {:>4}", "name", "category", "price", "qty")
///   per row: the same widths, price with 2 decimals: {:>8.2}
///   footer: "<count> rows, total <sum of price*qty with 2 decimals>"
pub fn report(rows: &[Row]) -> String {
    // TODO: start a Vec with the header line
    // extend it with rows.iter().map(|r| format!(...)), then push the footer
    // lines.join("\n")
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const CSV: &str = "name,category,price,qty\n\
        pen,office,1.50,10\n\
        stapler,office,8.00,2\n\
        desk,furniture,120.00,2\n\
        lamp,furniture,35.50,3\n";

    fn row(name: &str, category: &str, price: f64, qty: u32) -> Row {
        Row {
            name: name.to_string(),
            category: category.to_string(),
            price,
            qty,
        }
    }

    fn sample() -> Vec<Row> {
        vec![
            row("pen", "office", 1.5, 10),
            row("stapler", "office", 8.0, 2),
            row("desk", "furniture", 120.0, 2),
            row("lamp", "furniture", 35.5, 3),
        ]
    }

    #[test]
    fn row_total_multiplies_price_by_qty() {
        assert_eq!(row("pen", "office", 1.5, 10).total(), 15.0);
        assert_eq!(row("none", "office", 9.99, 0).total(), 0.0);
    }

    #[test]
    fn parse_line_reads_fields_and_rejects_bad_ones() {
        assert_eq!(
            parse_line("pen, office ,1.50, 10"),
            Ok(row("pen", "office", 1.5, 10)),
            "fields should be trimmed"
        );
        let e = parse_line("pen,office").unwrap_err();
        assert!(e.contains("4 fields"), "got: {e}");
        let e = parse_line("pen,office,abc,1").unwrap_err();
        assert!(e.contains("price"), "got: {e}");
        let e = parse_line("pen,office,1.0,-3").unwrap_err();
        assert!(e.contains("qty"), "got: {e}");
    }

    #[test]
    fn parse_rows_skips_header_and_blanks() {
        assert_eq!(parse_rows(CSV), Ok(sample()));
        let with_blanks = "name,category,price,qty\n\npen,office,1.50,10\n   \n";
        assert_eq!(parse_rows(with_blanks), Ok(vec![row("pen", "office", 1.5, 10)]));
        assert_eq!(parse_rows("name,category,price,qty"), Ok(vec![]));
    }

    #[test]
    fn parse_rows_reports_line_number() {
        let bad = "name,category,price,qty\n\npen,office,abc,1\nlamp,home,2.0,1\n";
        let e = parse_rows(bad).unwrap_err();
        assert!(e.starts_with("line 3:"), "blank lines count; got: {e}");
        assert!(e.contains("price"), "got: {e}");
        let short = "name,category,price,qty\npen,office,1.0,1\ndesk,furniture\n";
        let e = parse_rows(short).unwrap_err();
        assert!(e.starts_with("line 3:"), "got: {e}");
    }

    #[test]
    fn total_by_category_sums_per_category() {
        let totals = total_by_category(&sample());
        assert_eq!(totals.get("office"), Some(&31.0));
        assert_eq!(totals.get("furniture"), Some(&346.5));
        let keys: Vec<&str> = totals.keys().map(|k| k.as_str()).collect();
        assert_eq!(keys, vec!["furniture", "office"], "BTreeMap sorts the keys");
        assert!(total_by_category(&[]).is_empty());
    }

    #[test]
    fn top_n_picks_biggest_totals() {
        let rows = sample();
        let names: Vec<&str> = top_n(&rows, 2).iter().map(|r| r.name.as_str()).collect();
        assert_eq!(names, vec!["desk", "lamp"]);
        assert_eq!(top_n(&rows, 99).len(), 4, "asking for more than exist is fine");
        assert!(top_n(&rows, 0).is_empty());
    }

    #[test]
    fn cheapest_in_filters_a_category() {
        let rows = sample();
        assert_eq!(cheapest_in(&rows, "furniture").map(|r| r.name.as_str()), Some("lamp"));
        assert_eq!(cheapest_in(&rows, "office").map(|r| r.name.as_str()), Some("pen"));
        assert_eq!(cheapest_in(&rows, "toys"), None);
    }

    #[test]
    fn report_aligns_columns() {
        let rows = vec![row("pen", "office", 1.5, 10), row("desk", "furniture", 120.0, 2)];
        let expected = "\
name       category     price  qty
pen        office        1.50   10
desk       furniture   120.00    2
2 rows, total 255.00";
        assert_eq!(report(&rows), expected);
        let empty = report(&[]);
        assert!(
            empty.starts_with("name       category     price  qty\n0 rows"),
            "got: {empty}"
        );
    }
}
