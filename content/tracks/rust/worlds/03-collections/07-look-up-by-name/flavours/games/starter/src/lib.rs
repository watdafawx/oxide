use std::collections::HashMap;

/// A librarian's trades in emeralds: bread 1, arrow 2, diamond_sword 30.
pub fn villager_prices() -> HashMap<String, u32> {
    // TODO: make an empty map, insert the three prices, hand the map back
    HashMap::new()
}

/// What the villager wants for `item`, or None if they do not trade it.
pub fn trade_price(prices: &HashMap<String, u32>, item: &str) -> Option<u32> {
    // TODO: look the item up and turn Option<&u32> into Option<u32>
    todo!()
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
