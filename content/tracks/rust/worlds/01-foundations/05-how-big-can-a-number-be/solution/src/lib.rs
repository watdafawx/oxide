/// The change in a stock: stock_change(10, 4) -> -6 (six items left the shelf).
pub fn stock_change(before: i32, after: i32) -> i32 {
    after - before
}

/// The code printed on a shelf label. Codes run from 0 to 255.
pub fn shelf_code() -> u8 {
    200
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn taking_stock_out_is_negative() {
        assert_eq!(stock_change(10, 4), -6);
        assert_eq!(stock_change(4, 10), 6);
    }

    #[test]
    fn a_shelf_code_can_reach_200() {
        assert_eq!(shelf_code(), 200);
    }
}
