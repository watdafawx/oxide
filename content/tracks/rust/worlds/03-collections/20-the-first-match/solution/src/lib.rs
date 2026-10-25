/// The first price at or under `max`: ([2500, 400, 150], 500) -> Some(400)
pub fn first_affordable(prices: &[u32], max: u32) -> Option<u32> {
    prices.iter().copied().find(|p| *p <= max)
}

/// Is any product sold out (a count of 0)?
pub fn any_sold_out(counts: &[u32]) -> bool {
    counts.iter().any(|n| *n == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_affordable_finds_the_first_cheap_price() {
        assert_eq!(first_affordable(&[2500, 400, 150], 500), Some(400), "400 comes before 150");
        assert_eq!(first_affordable(&[2500], 500), None);
    }

    #[test]
    fn any_sold_out_checks_for_a_zero() {
        assert!(any_sold_out(&[12, 0, 6]));
        assert!(!any_sold_out(&[12, 6]));
        assert!(!any_sold_out(&[]), "an empty shelf has nothing sold out");
    }
}
