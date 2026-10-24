/// Remove every stack smaller than `min`: ([64, 1, 16, 2], 3) -> [64, 16]
pub fn trash_small_stacks(stacks: &mut Vec<u32>, min: u32) {
    stacks.retain(|n| *n >= min);
}

/// Remove every stack named `junk`: ([sword, flesh, bow], "flesh") -> [sword, bow]
pub fn drop_junk(items: &mut Vec<String>, junk: &str) {
    items.retain(|i| i != junk);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trash_small_stacks_keeps_the_big_ones() {
        let mut stacks = vec![64, 1, 16, 2];
        trash_small_stacks(&mut stacks, 3);
        assert_eq!(stacks, vec![64, 16]);
    }

    #[test]
    fn drop_junk_removes_one_item() {
        let mut items = vec!["sword".to_string(), "flesh".to_string(), "bow".to_string(), "flesh".to_string()];
        drop_junk(&mut items, "flesh");
        assert_eq!(items, vec!["sword", "bow"], "every stack of flesh goes");
    }
}
