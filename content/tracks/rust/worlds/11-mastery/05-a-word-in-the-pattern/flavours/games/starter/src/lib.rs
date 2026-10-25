#[derive(Debug, PartialEq)]
pub struct Block {
    pub name: &'static str,
    pub id: u16,
}

/// `block!("stone" => 1)` is `Block { name: "stone", id: 1 }`.
#[macro_export]
macro_rules! block {
    ($name:expr => $id:expr) => {
        // TODO: build a Block from the two captures (this stub ignores them)
        Block { name: "", id: 0 }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_builds_from_name_and_id() {
        assert_eq!(block!("stone" => 1), Block { name: "stone", id: 1 });
        assert_eq!(block!("dirt" => 1 + 2), Block { name: "dirt", id: 3 });
    }
}
