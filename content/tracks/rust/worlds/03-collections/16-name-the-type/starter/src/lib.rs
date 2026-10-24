/// The average price, rounded down: [100, 200, 600] -> 300. An empty list gives 0.
pub fn average_price(prices: &[u32]) -> u32 {
    if prices.len() == 0 {
        return 0;
    }
    // TODO: sum the prices as a u32 (say the type on the sum) and divide by the number of prices
    0
}

/// The total in euros: [150, 400] cents -> 5.5 euros.
pub fn total_in_euros(cents: &[u32]) -> f64 {
    // TODO: sum the cents as a u32, turn the sum into an f64 and divide by 100.0
    0.0
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
