use std::collections::HashSet;

/// How many different products were sold? [pen, mug, pen] -> 2
pub fn distinct_products(sales: &[&str]) -> usize {
    sales.iter().collect::<HashSet<_>>().len()
}

/// Products both shops stock, a to z, each once: ([pen, mug, lamp], [lamp, pen, sofa]) -> [lamp, pen]
pub fn in_both(shop_a: &[&str], shop_b: &[&str]) -> Vec<String> {
    let a: HashSet<&str> = shop_a.iter().copied().collect();
    let b: HashSet<&str> = shop_b.iter().copied().collect();
    let mut both: Vec<String> = a.intersection(&b).map(|p| p.to_string()).collect();
    both.sort();
    both
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
