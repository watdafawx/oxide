/// One line of the stock sheet.
#[derive(Debug, Clone, PartialEq)]
pub struct StockLine {
    pub name: String,
    pub qty: u32,
}

/// The most units first: pen 12, mug 6, lamp 1.
pub fn most_first(lines: &mut Vec<StockLine>) {
    // TODO: sort by qty, biggest first (compare b with a)
}

/// By name, a to z: lamp, mug, pen.
pub fn by_name(lines: &mut Vec<StockLine>) {
    // TODO: sort by name, smallest first (compare a with b)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn line(name: &str, qty: u32) -> StockLine {
        StockLine { name: name.to_string(), qty }
    }

    fn sheet() -> Vec<StockLine> {
        vec![line("mug", 6), line("lamp", 1), line("pen", 12)]
    }

    #[test]
    fn most_first_sorts_by_qty_descending() {
        let mut lines = sheet();
        most_first(&mut lines);
        assert_eq!(lines, vec![line("pen", 12), line("mug", 6), line("lamp", 1)]);
    }

    #[test]
    fn by_name_sorts_a_to_z() {
        let mut lines = sheet();
        by_name(&mut lines);
        assert_eq!(lines, vec![line("lamp", 1), line("mug", 6), line("pen", 12)]);
    }
}
