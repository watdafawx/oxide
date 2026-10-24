/// Name padded to 10 on the left, qty right-aligned in 5: ("pen", 12) -> "pen          12"
pub fn row(name: &str, qty: u32) -> String {
    // TODO: pad the name to 10 (left), then the qty to 5 (right)
    format!("{name}{qty}")
}

/// The price with 2 decimals, right-aligned in 8: 1.5 -> "    1.50"
pub fn price_cell(price: f64) -> String {
    // TODO: two decimals, right-aligned in a width of 8
    format!("{price}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_pads_name_and_qty() {
        assert_eq!(row("pen", 12), "pen          12");
        assert_eq!(row("notebook", 3), "notebook      3");
    }

    #[test]
    fn price_cell_has_two_decimals() {
        assert_eq!(price_cell(1.5), "    1.50");
        assert_eq!(price_cell(120.0), "  120.00");
        assert_eq!(price_cell(3.14159), "    3.14");
    }
}
