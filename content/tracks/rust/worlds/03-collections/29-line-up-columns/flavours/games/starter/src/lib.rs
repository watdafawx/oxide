/// Name padded to 12 on the left, count right-aligned in 4: ("diamond", 12) -> "diamond       12"
pub fn inventory_row(name: &str, count: u32) -> String {
    // TODO: pad the name to 12 (left), then the count to 4 (right)
    format!("{name}{count}")
}

/// The jar size with 1 decimal, right-aligned in 7: 1.2 -> "    1.2"
pub fn size_cell(size_mb: f64) -> String {
    // TODO: one decimal, right-aligned in a width of 7
    format!("{size_mb}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_row_pads_name_and_count() {
        assert_eq!(inventory_row("diamond", 12), "diamond       12");
        assert_eq!(inventory_row("torch", 3), "torch          3");
    }

    #[test]
    fn size_cell_has_one_decimal() {
        assert_eq!(size_cell(1.2), "    1.2");
        assert_eq!(size_cell(20.0), "   20.0");
        assert_eq!(size_cell(0.46), "    0.5");
    }
}
