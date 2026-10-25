/// A product on the price list. Prices are in cents.
#[derive(Debug, Clone, PartialEq)]
pub struct Product {
    pub name: String,
    pub price: u32,
}

/// Every price goes up by `percent` percent, in the caller's list: 200 with 10% -> 220.
pub fn raise_prices(products: &mut Vec<Product>, percent: u32) {
    // TODO: walk the products so you can change them, and add `percent` percent to each price
}

/// The names, in order. The products are not needed afterwards, so take them over.
pub fn into_names(products: Vec<Product>) -> Vec<String> {
    // TODO: hand the products over to a chain and keep only each one's name
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn product(name: &str, price: u32) -> Product {
        Product { name: name.to_string(), price }
    }

    #[test]
    fn raise_prices_changes_the_list_in_place() {
        let mut products = vec![product("pen", 200), product("lamp", 2500)];
        raise_prices(&mut products, 10);
        assert_eq!(products, vec![product("pen", 220), product("lamp", 2750)]);
    }

    #[test]
    fn into_names_takes_the_names_without_copying() {
        let products = vec![product("pen", 150), product("mug", 400)];
        assert_eq!(into_names(products), vec!["pen", "mug"]);
        assert_eq!(into_names(Vec::new()), Vec::<String>::new());
    }
}
