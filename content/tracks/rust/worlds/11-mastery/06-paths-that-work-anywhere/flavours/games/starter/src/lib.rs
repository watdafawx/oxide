#[derive(Debug, PartialEq)]
pub struct Block {
    pub name: &'static str,
    pub id: u16,
}

/// `block!("stone" => 1)` is `Block { name: "stone", id: 1 }`, from any module.
#[macro_export]
macro_rules! block {
    // TODO: name the struct by a path that works wherever the macro is called
    ($name:expr => $id:expr) => {
        Block { name: $name, id: $id }
    };
}

/// `id_map!("stone" => 1)` is a HashMap holding that one pair, from any module.
#[macro_export]
macro_rules! id_map {
    // TODO: name HashMap by its full path, starting from the std crate
    ($name:expr => $id:expr) => {{
        let mut map = HashMap::new();
        map.insert($name, $id);
        map
    }};
}

/// This module imports nothing, like a user's file that only calls the macros.
#[cfg(test)]
mod elsewhere {
    #[test]
    fn block_works_from_another_module() {
        let stone = crate::block!("stone" => 1);
        assert_eq!(stone.name, "stone");
        assert_eq!(stone.id, 1);
    }

    #[test]
    fn id_map_works_without_imports() {
        let map = crate::id_map!("dirt" => 2);
        assert_eq!(map.len(), 1);
        assert_eq!(map["dirt"], 2);
    }
}
