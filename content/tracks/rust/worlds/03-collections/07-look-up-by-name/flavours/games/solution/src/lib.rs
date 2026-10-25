use std::collections::HashMap;

/// A librarian's trades in emeralds: bread 1, arrow 2, diamond_sword 30.
pub fn villager_prices() -> HashMap<String, u32> {
    let mut prices = HashMap::new();
    prices.insert("bread".to_string(), 1);
    prices.insert("arrow".to_string(), 2);
    prices.insert("diamond_sword".to_string(), 30);
    prices
}

/// What the villager wants for `item`, or None if they do not trade it.
pub fn trade_price(prices: &HashMap<String, u32>, item: &str) -> Option<u32> {
    prices.get(item).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn villager_prices_hold_three_trades() {
        let prices = villager_prices();
        assert_eq!(prices.len(), 3);
        assert_eq!(prices.get("bread"), Some(&1));
        assert_eq!(prices.get("arrow"), Some(&2));
        assert_eq!(prices.get("diamond_sword"), Some(&30));
    }

    #[test]
    fn trade_price_finds_known_items() {
        let mut prices = HashMap::new();
        prices.insert("emerald_block".to_string(), 9);
        assert_eq!(trade_price(&prices, "emerald_block"), Some(9));
        assert_eq!(trade_price(&prices, "elytra"), None, "nobody trades elytra");
    }
}
