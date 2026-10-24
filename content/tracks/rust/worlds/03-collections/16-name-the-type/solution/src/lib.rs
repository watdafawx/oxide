/// The average price, rounded down: [100, 200, 600] -> 300. An empty list gives 0.
pub fn average_price(prices: &[u32]) -> u32 {
    if prices.len() == 0 {
        return 0;
    }
    prices.iter().sum::<u32>() / prices.len() as u32
}

/// The total in euros: [150, 400] cents -> 5.5 euros.
pub fn total_in_euros(cents: &[u32]) -> f64 {
    cents.iter().sum::<u32>() as f64 / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn average_price_divides_the_sum() {
        assert_eq!(average_price(&[100, 200, 600]), 300);
        assert_eq!(average_price(&[150, 400]), 275);
        assert_eq!(average_price(&[]), 0, "no prices, no average");
    }

    #[test]
    fn total_in_euros_turns_cents_into_euros() {
        assert_eq!(total_in_euros(&[150, 400]), 5.5);
        assert_eq!(total_in_euros(&[2500]), 25.0);
    }
}
