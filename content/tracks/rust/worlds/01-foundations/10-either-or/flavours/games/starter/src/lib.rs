/// A tool stacks to 1, any other item to 64.
pub fn stack_limit(is_tool: bool) -> u32 {
    0 // TODO: an `if is_tool { ... } else { ... }` with no semicolon
}

/// Coal for smelting: up to 8 items need 1, up to 16 need 2, more need 3.
pub fn coal_needed(items: u32) -> u32 {
    0 // TODO: if items <= 8 ... else if items <= 16 ... else ...
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tools_stack_alone() {
        assert_eq!(stack_limit(true), 1);
        assert_eq!(stack_limit(false), 64);
    }

    #[test]
    fn coal_grows_with_items() {
        assert_eq!(coal_needed(8), 1);
        assert_eq!(coal_needed(9), 2);
        assert_eq!(coal_needed(16), 2);
        assert_eq!(coal_needed(17), 3);
    }
}
