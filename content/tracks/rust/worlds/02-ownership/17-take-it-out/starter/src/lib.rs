/// Takes the first item called `wanted` out of the list and hands it over.
/// None if there is no such item; the list is then unchanged.
pub fn take(items: &mut Vec<String>, wanted: &str) -> Option<String> {
    // TODO: find the position of `wanted`, remove that item and give it back;
    // None when it isn't there
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bag() -> Vec<String> {
        vec!["rope".to_string(), "lamp".to_string(), "map".to_string()]
    }

    #[test]
    fn take_hands_over_the_item() {
        let mut items = bag();
        assert_eq!(take(&mut items, "lamp"), Some("lamp".to_string()));
        assert_eq!(items, vec!["rope", "map"], "the lamp must leave the list");
    }

    #[test]
    fn take_missing_gives_none() {
        let mut items = bag();
        assert_eq!(take(&mut items, "sword"), None);
        assert_eq!(items, vec!["rope", "lamp", "map"]);
    }
}
