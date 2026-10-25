/// A product on the price list. Prices are in cents.
#[derive(Debug, Clone, PartialEq)]
pub struct Product {
    pub name: String,
    pub price: u32,
}

/// The prices at or under `max`: ([150, 2500, 400], 500) -> [150, 400]
pub fn cheap_prices(prices: &[u32], max: u32) -> Vec<u32> {
    prices.iter().copied().filter(|p| *p <= max).collect()
}

/// The products whose price is at or under `max`, as references into the list.
pub fn affordable(products: &[Product], max: u32) -> Vec<&Product> {
    products.iter().filter(|p| p.price <= max).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn product(name: &str, price: u32) -> Product {
        Product { name: name.to_string(), price }
    }

    #[test]
    fn cheap_prices_keeps_prices_up_to_max() {
        assert_eq!(cheap_prices(&[150, 2500, 400], 500), vec![150, 400]);
        assert_eq!(cheap_prices(&[2500], 500), Vec::<u32>::new());
    }

    #[test]
    fn affordable_keeps_products_up_to_max() {
        let products = vec![product("pen", 150), product("lamp", 2500), product("mug", 400)];
        let got = affordable(&products, 500);
        assert_eq!(got, vec![&products[0], &products[2]], "pen and mug, in list order");
        assert!(affordable(&products, 100).is_empty());
    }
}
