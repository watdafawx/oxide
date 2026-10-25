#[derive(Debug, PartialEq)]
pub struct Item {
    pub name: &'static str,
    pub count: u32,
}

/// `item!("apples" => 3)` is `Item { name: "apples", count: 3 }`.
#[macro_export]
macro_rules! item {
    ($name:expr => $count:expr) => {
        // TODO: build an Item from the two captures (this stub ignores them)
        Item { name: "", count: 0 }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_builds_from_name_and_count() {
        assert_eq!(item!("apples" => 3), Item { name: "apples", count: 3 });
        assert_eq!(item!("pears" => 1 + 1), Item { name: "pears", count: 2 });
    }
}
