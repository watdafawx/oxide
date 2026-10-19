

/// ["sword", "bow"] -> ["[1] sword", "[2] bow"]. Hotbar slots count from 1.
pub fn hotbar_labels(items: &[&str]) -> Vec<String> {
    // TODO: enumerate gives (i, item); format!("[{}] {}", i + 1, item) makes each label
    todo!()
}

/// Counts [2, 3, 1] priced [10, 5, 100] is worth 2*10 + 3*5 + 1*100 = 135 emeralds.
pub fn loot_value(counts: &[u32], prices: &[u32]) -> u32 {
    // TODO: counts.iter().zip(prices) gives pairs; multiply each pair, then sum
    todo!()
}

/// Pages start at 0. chest_page(&["a","b","c","d","e"], 1, 2) is ["c", "d"].
pub fn chest_page(items: &[&str], page: usize, size: usize) -> Vec<String> {
    // TODO: skip(page * size) then take(size), then map to String and collect
    todo!()
}

/// Tiers [1, 3, 2, 4] along the hotbar: how many times is the next slot a higher tier? -> 2
pub fn count_upgrades(tiers: &[u32]) -> usize {
    // TODO: tiers.windows(2) gives slices like [1, 3]; filter where pair[1] > pair[0]
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbered_uses_enumerate() {
        assert_eq!(hotbar_labels(&["sword", "bow"]), vec!["[1] sword", "[2] bow"]);
        assert_eq!(hotbar_labels(&[]), Vec::<String>::new());
    }

    #[test]
    fn dot_product_zips_two_slices() {
        assert_eq!(loot_value(&[2, 3, 1], &[10, 5, 100]), 135);
        assert_eq!(loot_value(&[1, 2, 3], &[10, 10]), 30, "stops at the shorter slice");
    }

    #[test]
    fn page_skips_then_takes() {
        let items = ["a", "b", "c", "d", "e"];
        assert_eq!(chest_page(&items, 0, 2), vec!["a", "b"]);
        assert_eq!(chest_page(&items, 1, 2), vec!["c", "d"]);
        assert_eq!(chest_page(&items, 2, 2), vec!["e"], "last page can be short");
        assert_eq!(chest_page(&items, 9, 2), Vec::<String>::new());
    }

    #[test]
    fn count_rises_uses_windows() {
        assert_eq!(count_upgrades(&[1, 3, 2, 4]), 2);
        assert_eq!(count_upgrades(&[4, 3, 2]), 0);
        assert_eq!(count_upgrades(&[7]), 0);
        assert_eq!(count_upgrades(&[]), 0);
    }
}
