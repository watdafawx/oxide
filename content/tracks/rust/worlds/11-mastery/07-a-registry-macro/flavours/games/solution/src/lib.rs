#[derive(Debug, PartialEq)]
pub struct Block {
    pub name: &'static str,
    pub id: u16,
}

/// `block!("stone" => 1)` is `Block { name: "stone", id: 1 }`.
#[macro_export]
macro_rules! block {
    ($name:expr => $id:expr) => {
        $crate::Block { name: $name, id: $id }
    };
}

/// `registry!["stone" => 1, "dirt" => 2]` is a `Vec<Block>` with both blocks, in order.
#[macro_export]
macro_rules! registry {
    ($($name:expr => $id:expr),* $(,)?) => {
        vec![$( $crate::block!($name => $id) ),*]
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_builds_every_block() {
        let blocks = registry![
            "stone" => 1,
            "dirt" => 2,
        ];
        assert_eq!(
            blocks,
            vec![Block { name: "stone", id: 1 }, Block { name: "dirt", id: 2 }]
        );
    }

    #[test]
    fn registry_can_be_empty() {
        let none: Vec<Block> = registry![];
        assert!(none.is_empty());
    }
}
