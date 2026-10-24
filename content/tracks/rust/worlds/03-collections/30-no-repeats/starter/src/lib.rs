use std::collections::HashSet;

/// How many different products were sold? [pen, mug, pen] -> 2
pub fn distinct_products(sales: &[&str]) -> usize {
    // TODO: collect the sales into a HashSet (repeats vanish) and give its length
    sales.len()
}

/// Products both shops stock, a to z, each once: ([pen, mug, lamp], [lamp, pen, sofa]) -> [lamp, pen]
pub fn in_both(shop_a: &[&str], shop_b: &[&str]) -> Vec<String> {
    // TODO: make a HashSet of each list, intersect them, map each to a String, collect, then sort
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distinct_products_ignores_repeats() {
        assert_eq!(distinct_products(&["pen", "mug", "pen", "pen"]), 2);
        assert_eq!(distinct_products(&[]), 0);
    }

    #[test]
    fn in_both_lists_shared_products() {
        assert_eq!(in_both(&["pen", "mug", "lamp", "pen"], &["lamp", "pen", "sofa"]), vec!["lamp", "pen"]);
        assert_eq!(in_both(&["pen"], &["sofa"]), Vec::<String>::new());
    }
}
