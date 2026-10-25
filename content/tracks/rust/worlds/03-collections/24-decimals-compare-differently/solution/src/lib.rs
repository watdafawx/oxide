/// An item from the supplier's list, with a decimal price in euros.
#[derive(Debug, Clone, PartialEq)]
pub struct PricedItem {
    pub name: String,
    pub price: f64,
}

/// The prices from lowest to highest: [35.5, 1.5, 8.0] -> [1.5, 8.0, 35.5]
pub fn sorted_prices(prices: Vec<f64>) -> Vec<f64> {
    let mut prices = prices;
    prices.sort_by(|a, b| a.total_cmp(b));
    prices
}

/// The item with the lowest price. None for an empty list.
pub fn cheapest(items: &[PricedItem]) -> Option<&PricedItem> {
    items.iter().min_by(|a, b| a.price.total_cmp(&b.price))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(name: &str, price: f64) -> PricedItem {
        PricedItem { name: name.to_string(), price }
    }

    #[test]
    fn sorted_prices_go_low_to_high() {
        assert_eq!(sorted_prices(vec![35.5, 1.5, 8.0]), vec![1.5, 8.0, 35.5]);
    }

    #[test]
    fn cheapest_picks_the_lowest_price() {
        let items = vec![item("lamp", 35.5), item("pen", 1.5), item("stapler", 8.0)];
        assert_eq!(cheapest(&items), Some(&items[1]));
        assert_eq!(cheapest(&[]), None);
    }
}
