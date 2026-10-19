pub enum ItemKind {
    Block { hardness: u32 },
    Tool(u32),
    Food { servings: u32 },
}

impl ItemKind {
    /// How many uses this item gives: Tool(durability) -> durability,
    /// Food { servings } -> servings, Block -> 1 (you place it once).
    pub fn uses(&self) -> u32 {
        // TODO: match self, binding the data of each variant
        todo!()
    }

    /// "block", "tool" or "food"
    pub fn name(&self) -> &'static str {
        // TODO: match self; use `_` or `{ .. }` for data you do not need
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tool_uses_equal_durability() {
        let uses = ItemKind::Tool(250).uses();
        assert_eq!(uses, 250, "a tool gives as many uses as its durability, got {uses}");
    }

    #[test]
    fn food_and_block_have_uses() {
        let food = ItemKind::Food { servings: 3 }.uses();
        let block = ItemKind::Block { hardness: 2 }.uses();
        assert_eq!(food, 3, "Food with 3 servings should give 3 uses, got {food}");
        assert_eq!(block, 1, "a block is placed once, got {block}");
    }

    #[test]
    fn item_kind_names_are_block_tool_food() {
        assert_eq!(ItemKind::Block { hardness: 1 }.name(), "block");
        assert_eq!(ItemKind::Tool(10).name(), "tool");
        assert_eq!(ItemKind::Food { servings: 1 }.name(), "food");
    }
}
