/// Pens, mugs and lamps on the shelf when the shop opens: 12, 6 and 1.
pub fn opening_stock() -> Vec<u32> {
    // TODO: write the three counts with vec!
    todo!()
}

/// The opening stock, plus a new product (20 notebooks) on the end.
pub fn after_new_product() -> Vec<u32> {
    // TODO: start from opening_stock(), push 20, hand the list back
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opening_stock_lists_three_counts() {
        assert_eq!(opening_stock(), vec![12, 6, 1]);
    }

    #[test]
    fn a_new_product_goes_on_the_end() {
        assert_eq!(after_new_product(), vec![12, 6, 1, 20], "20 notebooks go last");
    }
}
