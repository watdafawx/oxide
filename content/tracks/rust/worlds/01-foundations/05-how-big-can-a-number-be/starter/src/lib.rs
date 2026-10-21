/// The change in a stock: stock_change(10, 4) -> -6 (six items left the shelf).
// TODO: a change can be negative. Change the return type.
pub fn stock_change(before: i32, after: i32) -> u32 {
    after - before
}

/// The code printed on a shelf label. Codes run from 0 to 255.
// TODO: 200 does not fit in an `i8`. Read the error and pick the right kind.
pub fn shelf_code() -> i8 {
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
