use std::collections::HashMap;

/// Prices in cents: pen 150, mug 400, lamp 2500.
pub fn price_list() -> HashMap<String, u32> {
    let mut prices = HashMap::new();
    prices.insert("pen".to_string(), 150);
    prices.insert("mug".to_string(), 400);
    prices.insert("lamp".to_string(), 2500);
    prices
}

/// The price of `product`, or None if the shop does not sell it.
pub fn price_of(prices: &HashMap<String, u32>, product: &str) -> Option<u32> {
    prices.get(product).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn price_list_holds_three_prices() {
        let prices = price_list();
        assert_eq!(prices.len(), 3);
        assert_eq!(prices.get("pen"), Some(&150));
        assert_eq!(prices.get("mug"), Some(&400));
        assert_eq!(prices.get("lamp"), Some(&2500));
    }

    #[test]
    fn price_of_finds_known_products() {
        let mut prices = HashMap::new();
        prices.insert("stapler".to_string(), 800);
        assert_eq!(price_of(&prices, "stapler"), Some(800));
        assert_eq!(price_of(&prices, "sofa"), None, "the shop does not sell sofas");
    }
}
