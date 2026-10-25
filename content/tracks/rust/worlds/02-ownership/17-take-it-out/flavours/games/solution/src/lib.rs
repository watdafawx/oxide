/// Takes the first `item` out of the admin stash and hands it over.
/// None if the stash has no such item; the stash is then unchanged.
pub fn take_from_stash(stash: &mut Vec<String>, item: &str) -> Option<String> {
    for i in 0..stash.len() {
        if stash[i] == item {
            return Some(stash.remove(i));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stash() -> Vec<String> {
        vec!["torch".to_string(), "diamond".to_string(), "bread".to_string()]
    }

    #[test]
    fn take_from_stash_hands_over_the_item() {
        let mut s = stash();
        assert_eq!(take_from_stash(&mut s, "diamond"), Some("diamond".to_string()));
        assert_eq!(s, vec!["torch", "bread"], "the diamond must leave the stash");
    }

    #[test]
    fn take_missing_item_gives_none() {
        let mut s = stash();
        assert_eq!(take_from_stash(&mut s, "elytra"), None);
        assert_eq!(s, vec!["torch", "diamond", "bread"]);
    }
}
