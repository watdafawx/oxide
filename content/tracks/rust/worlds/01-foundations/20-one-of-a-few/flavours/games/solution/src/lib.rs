pub enum ItemKind {
    Block,
    Tool,
    Food,
}

/// Stack sizes: blocks 64, tools 1, food 16.
pub fn max_stack(kind: ItemKind) -> u32 {
    match kind {
        ItemKind::Block => 64,
        ItemKind::Tool => 1,
        ItemKind::Food => 16,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocks_and_food_stack() {
        assert_eq!(max_stack(ItemKind::Block), 64);
        assert_eq!(max_stack(ItemKind::Food), 16);
    }

    #[test]
    fn tools_stack_alone() {
        assert_eq!(max_stack(ItemKind::Tool), 1);
    }
}
