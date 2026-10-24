/// The orders, a to z. [mug, pen, lamp] -> [lamp, mug, pen].
pub fn sorted_orders(orders: Vec<String>) -> Vec<String> {
    let mut orders = orders;
    orders.sort();
    orders
}

/// Each product once, a to z. [mug, pen, mug, lamp, pen] -> [lamp, mug, pen].
pub fn unique_orders(orders: Vec<String>) -> Vec<String> {
    let mut orders = orders;
    orders.sort();
    orders.dedup();
    orders
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names(items: &[&str]) -> Vec<String> {
        let mut out = Vec::new();
        for item in items {
            out.push(item.to_string());
        }
        out
    }

    #[test]
    fn sorted_orders_are_a_to_z() {
        assert_eq!(sorted_orders(names(&["mug", "pen", "lamp"])), names(&["lamp", "mug", "pen"]));
    }

    #[test]
    fn unique_orders_have_no_repeats() {
        assert_eq!(
            unique_orders(names(&["mug", "pen", "mug", "lamp", "pen"])),
            names(&["lamp", "mug", "pen"]),
            "the two mugs are not neighbours until you sort"
        );
        assert_eq!(unique_orders(Vec::new()), Vec::<String>::new());
    }
}
