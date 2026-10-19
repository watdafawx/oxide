#[derive(Debug, PartialEq)]
pub struct Block {
    pub name: &'static str,
    pub id: u16,
}

/// `block!("stone" => 1)` is `Block { name: "stone", id: 1 }`.
#[macro_export]
macro_rules! block {
    ($name:expr => $id:expr) => {{
        // TODO: return `$crate::Block { name: $name, id: $id }`.
        // (This stub ignores the arguments, so the crate compiles.)
        let _ = ($name, $id);
        $crate::Block { name: "", id: 0 }
    }};
}

/// `registry!["stone" => 1, "dirt" => 2]` is a `Vec<Block>` with both blocks.
#[macro_export]
macro_rules! registry {
    ($($name:expr => $id:expr),* $(,)?) => {{
        // TODO: return `vec![$( $crate::block!($name => $id) ),*]`.
        // (This stub ignores the arguments, so the crate compiles.)
        $(let _ = ($name, $id);)*
        Vec::<$crate::Block>::new()
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_macro_builds_a_block() {
        assert_eq!(block!("stone" => 1), Block { name: "stone", id: 1 });
    }

    #[test]
    fn registry_macro_builds_every_block() {
        let blocks = registry!["stone" => 1, "dirt" => 2,];
        assert_eq!(
            blocks,
            vec![Block { name: "stone", id: 1 }, Block { name: "dirt", id: 2 }]
        );
        let none: Vec<Block> = registry![];
        assert!(none.is_empty());
    }
}
