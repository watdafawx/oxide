pub struct Backpack {
    pub items: Vec<String>,
}

/// How many items are in the bag.
pub fn count_items(bag: &Backpack) -> usize {
    // TODO: give back how many items the bag holds
    todo!()
}

/// The length of every item name, added up.
pub fn total_chars(bag: &Backpack) -> usize {
    // TODO: walk the items without taking them and add up their lengths
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bag() -> Backpack {
        Backpack {
            items: vec!["rope".to_string(), "lamp".to_string(), "map".to_string()],
        }
    }

    #[test]
    fn count_items_counts() {
        assert_eq!(count_items(&bag()), 3);
    }

    #[test]
    fn total_chars_adds_lengths() {
        let b = bag();
        assert_eq!(total_chars(&b), 11, "rope + lamp + map = 4 + 4 + 3");
        assert_eq!(b.items[0], "rope", "the bag must still be yours after lending it");
    }
}
