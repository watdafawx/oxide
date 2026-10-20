

/// ["sword", "bow"] -> ["[1] sword", "[2] bow"]. Hotbar slots count from 1.
pub fn hotbar_labels(items: &[&str]) -> Vec<String> {
    // TODO: label every item "[n] name", counting from 1
    todo!()
}

/// Counts [2, 3, 1] priced [10, 5, 100] is worth 2*10 + 3*5 + 1*100 = 135 emeralds.
pub fn loot_value(counts: &[u32], prices: &[u32]) -> u32 {
    // TODO: multiply each count by its price and add the products up
    todo!()
}

/// Pages start at 0. chest_page(&["a","b","c","d","e"], 1, 2) is ["c", "d"].
pub fn chest_page(items: &[&str], page: usize, size: usize) -> Vec<String> {
    // TODO: take one page of `size` items starting at `page * size`, as Strings
    todo!()
}

/// Tiers [1, 3, 2, 4] along the hotbar: how many times is the next slot a higher tier? -> 2
pub fn count_upgrades(tiers: &[u32]) -> usize {
    // TODO: look at each neighbouring pair of tiers and count the ones that go up
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hotbar_labels_are_numbered() {
        assert_eq!(hotbar_labels(&["sword", "bow"]), vec!["[1] sword", "[2] bow"]);
        assert_eq!(hotbar_labels(&[]), Vec::<String>::new());
    }

    #[test]
    fn loot_value_pairs_counts_with_prices() {
        assert_eq!(loot_value(&[2, 3, 1], &[10, 5, 100]), 135);
        assert_eq!(loot_value(&[1, 2, 3], &[10, 10]), 30, "stops at the shorter slice");
    }

    #[test]
    fn chest_page_skips_then_takes() {
        let items = ["a", "b", "c", "d", "e"];
        assert_eq!(chest_page(&items, 0, 2), vec!["a", "b"]);
        assert_eq!(chest_page(&items, 1, 2), vec!["c", "d"]);
        assert_eq!(chest_page(&items, 2, 2), vec!["e"], "last page can be short");
        assert_eq!(chest_page(&items, 9, 2), Vec::<String>::new());
    }

    #[test]
    fn count_upgrades_compares_neighbours() {
        assert_eq!(count_upgrades(&[1, 3, 2, 4]), 2);
        assert_eq!(count_upgrades(&[4, 3, 2]), 0);
        assert_eq!(count_upgrades(&[7]), 0);
        assert_eq!(count_upgrades(&[]), 0);
    }
}
