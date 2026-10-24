use std::collections::HashMap;

/// [pen, mug, pen, pen] -> pen: 3, mug: 1
pub fn count_sales(sales: &[&str]) -> HashMap<String, u32> {
    let mut counts = HashMap::new();
    for product in sales {
        // TODO: add 1 to this product's count, starting at 0 the first time
    }
    counts
}

/// A delivery of `amount` units. Known products go up; a new product starts at 0 first.
pub fn add_delivery(stock: &mut HashMap<String, u32>, product: &str, amount: u32) {
    // TODO: add `amount` to the count for `product`, starting at 0 if it is new
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_sales_tallies_each_product() {
        let counts = count_sales(&["pen", "mug", "pen", "pen"]);
        assert_eq!(counts.get("pen"), Some(&3));
        assert_eq!(counts.get("mug"), Some(&1));
        assert_eq!(counts.len(), 2);
        assert!(count_sales(&[]).is_empty(), "no sales, no counts");
    }

    #[test]
    fn add_delivery_raises_or_creates_the_count() {
        let mut stock = HashMap::new();
        stock.insert("pen".to_string(), 12);
        add_delivery(&mut stock, "pen", 30);
        add_delivery(&mut stock, "lamp", 2);
        assert_eq!(stock.get("pen"), Some(&42), "12 + 30");
        assert_eq!(stock.get("lamp"), Some(&2), "a new product starts at 0");
    }
}
