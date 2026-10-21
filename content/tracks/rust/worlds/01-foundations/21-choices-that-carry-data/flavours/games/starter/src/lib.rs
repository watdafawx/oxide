pub enum ItemKind {
    Block,
    Tool(u32),
    Food(u32),
}

/// How many uses an item gives: a block is placed once, a tool lasts its durability, food its servings.
pub fn uses(kind: ItemKind) -> u32 {
    // TODO: match kind, naming the data in the Tool and Food arms
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tools_use_their_durability() {
        assert_eq!(uses(ItemKind::Tool(250)), 250);
        assert_eq!(uses(ItemKind::Tool(32)), 32);
    }

    #[test]
    fn food_uses_its_servings_and_blocks_one() {
        assert_eq!(uses(ItemKind::Food(3)), 3);
        assert_eq!(uses(ItemKind::Block), 1);
    }
}
