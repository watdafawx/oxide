use std::collections::HashMap;

/// Prices in cents: pen 150, mug 400, lamp 2500.
pub fn price_list() -> HashMap<String, u32> {
    // TODO: make an empty map, insert the three prices, hand the map back
    HashMap::new()
}

/// The price of `product`, or None if the shop does not sell it.
pub fn price_of(prices: &HashMap<String, u32>, product: &str) -> Option<u32> {
    // TODO: look the product up and turn Option<&u32> into Option<u32>
    todo!()
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
