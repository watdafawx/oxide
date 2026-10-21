/// A stack holds 64 items. full_stacks(150) -> 2
pub fn full_stacks(items: u32) -> u32 {
    items / 64
}

/// Items that do not fill a stack. loose_items(150) -> 22
pub fn loose_items(items: u32) -> u32 {
    items % 64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_stacks_count_whole_stacks() {
        assert_eq!(full_stacks(150), 2);
        assert_eq!(full_stacks(63), 0);
        assert_eq!(full_stacks(64), 1);
    }

    #[test]
    fn loose_items_are_the_remainder() {
        assert_eq!(loose_items(150), 22);
        assert_eq!(loose_items(63), 63);
        assert_eq!(loose_items(128), 0, "an exact fit leaves nothing");
    }
}
