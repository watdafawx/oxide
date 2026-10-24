/// The first stack bigger than `limit`: ([4, 8, 15, 16], 10) -> Some(15)
pub fn first_stack_over(stacks: &[u32], limit: u32) -> Option<u32> {
    stacks.iter().copied().find(|n| *n > limit)
}

/// Is any tool one hit from breaking (durability 1 or less)?
pub fn has_fragile_tool(durability: &[u32]) -> bool {
    durability.iter().any(|d| *d <= 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_stack_over_finds_the_first_big_stack() {
        assert_eq!(first_stack_over(&[4, 8, 15, 16], 10), Some(15), "15 comes before 16");
        assert_eq!(first_stack_over(&[4, 8], 10), None);
    }

    #[test]
    fn has_fragile_tool_checks_for_a_worn_one() {
        assert!(has_fragile_tool(&[30, 1, 12]));
        assert!(!has_fragile_tool(&[30, 2, 12]));
        assert!(!has_fragile_tool(&[]), "no tools, nothing breaking");
    }
}
