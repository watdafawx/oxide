pub struct Backpack {
    pub items: Vec<String>,
}

/// How many items are in the bag.
pub fn count_items(bag: &Backpack) -> usize {
    // TODO: return the length of bag.items
    todo!()
}

/// Sum of the length of every item name.
pub fn total_chars(bag: &Backpack) -> usize {
    // TODO: add up item.len() for every item in bag.items
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
        assert_eq!(total_chars(&bag()), 11, "rope + lamp + map = 4 + 4 + 3");
    }

    #[test]
    fn bag_is_still_usable_after_borrowing() {
        let b = bag();
        let n = count_items(&b);
        let c = total_chars(&b);
        assert_eq!((n, c), (3, 11));
        assert_eq!(b.items[0], "rope", "the bag must still be ours");
    }
}
