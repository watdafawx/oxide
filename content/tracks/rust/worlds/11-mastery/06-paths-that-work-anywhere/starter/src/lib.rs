#[derive(Debug, PartialEq)]
pub struct Item {
    pub name: &'static str,
    pub count: u32,
}

/// `item!("apples" => 3)` is `Item { name: "apples", count: 3 }`, from any module.
#[macro_export]
macro_rules! item {
    // TODO: name the struct by a path that works wherever the macro is called
    ($name:expr => $count:expr) => {
        Item { name: $name, count: $count }
    };
}

/// `stock_map!("apples" => 3)` is a HashMap holding that one pair, from any module.
#[macro_export]
macro_rules! stock_map {
    // TODO: name HashMap by its full path, starting from the std crate
    ($name:expr => $count:expr) => {{
        let mut map = HashMap::new();
        map.insert($name, $count);
        map
    }};
}

/// This module imports nothing, like a user's file that only calls the macros.
#[cfg(test)]
mod elsewhere {
    #[test]
    fn item_works_from_another_module() {
        let apples = crate::item!("apples" => 3);
        assert_eq!(apples.name, "apples");
        assert_eq!(apples.count, 3);
    }

    #[test]
    fn stock_map_works_without_imports() {
        let map = crate::stock_map!("pears" => 2);
        assert_eq!(map.len(), 1);
        assert_eq!(map["pears"], 2);
    }
}
