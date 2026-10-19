pub enum ItemKind {
    Block { hardness: u32 },
    Tool(u32),
    Food { servings: u32 },
}

impl ItemKind {
    /// How many uses this item gives: Tool(durability) -> durability,
    /// Food { servings } -> servings, Block -> 1 (you place it once).
    pub fn uses(&self) -> u32 {
        match self {
            ItemKind::Tool(durability) => *durability,
            ItemKind::Food { servings } => *servings,
            ItemKind::Block { .. } => 1,
        }
    }

    /// "block", "tool" or "food"
    pub fn name(&self) -> &'static str {
        match self {
            ItemKind::Block { .. } => "block",
            ItemKind::Tool(_) => "tool",
            ItemKind::Food { .. } => "food",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle_area_uses_pi() {
        let uses = ItemKind::Tool(250).uses();
        assert_eq!(uses, 250, "a tool gives as many uses as its durability, got {uses}");
    }

    #[test]
    fn rect_and_square_areas() {
        let food = ItemKind::Food { servings: 3 }.uses();
        let block = ItemKind::Block { hardness: 2 }.uses();
        assert_eq!(food, 3, "Food with 3 servings should give 3 uses, got {food}");
        assert_eq!(block, 1, "a block is placed once, got {block}");
    }

    #[test]
    fn names_describe_each_variant() {
        assert_eq!(ItemKind::Block { hardness: 1 }.name(), "block");
        assert_eq!(ItemKind::Tool(10).name(), "tool");
        assert_eq!(ItemKind::Food { servings: 1 }.name(), "food");
    }
}
