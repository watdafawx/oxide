#[derive(Debug, PartialEq)]
pub struct Item {
    pub name: &'static str,
    pub count: u32,
}

/// `item!("apples" => 3)` is `Item { name: "apples", count: 3 }`.
#[macro_export]
macro_rules! item {
    ($name:expr => $count:expr) => {
        $crate::Item { name: $name, count: $count }
    };
}

/// `stock!["apples" => 3, "pears" => 2]` is a `Vec<Item>` with both items, in order.
#[macro_export]
macro_rules! stock {
    ($($name:expr => $count:expr),* $(,)?) => {
        // TODO: a vec! with one item! call (through $crate) per captured pair, separated by commas.
        // (This stub ignores the arguments, so the crate compiles.)
        Vec::<$crate::Item>::new()
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stock_builds_every_item() {
        let shop = stock![
            "apples" => 3,
            "pears" => 2,
        ];
        assert_eq!(
            shop,
            vec![Item { name: "apples", count: 3 }, Item { name: "pears", count: 2 }]
        );
    }

    #[test]
    fn stock_can_be_empty() {
        let none: Vec<Item> = stock![];
        assert!(none.is_empty());
    }
}
