/// Take each percent off what is left, in turn: 200.0 with [10.0, 50.0] -> 200 * 0.9 * 0.5 = 90.0
pub fn after_discounts(price: f64, percents: &[f64]) -> f64 {
    percents.iter().fold(price, |left, off| left * (1.0 - off / 100.0))
}

/// Take every sale off the start count, one after the other, never going below 0.
pub fn stock_after_sales(start: u32, sales: &[u32]) -> u32 {
    sales.iter().fold(start, |left, n| left.saturating_sub(*n))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn after_discounts_applies_each_in_turn() {
        let got = after_discounts(200.0, &[10.0, 50.0]);
        assert!((got - 90.0).abs() < 1e-9, "expected 90.0, got {got}");
        assert_eq!(after_discounts(80.0, &[]), 80.0, "no discounts, same price");
    }

    #[test]
    fn stock_after_sales_never_goes_below_zero() {
        assert_eq!(stock_after_sales(12, &[3, 4]), 5);
        assert_eq!(stock_after_sales(5, &[3, 4, 1]), 0, "stops at 0");
    }
}
