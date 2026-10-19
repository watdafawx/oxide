

/// ["sword", "bow"] -> ["[1] sword", "[2] bow"]. Hotbar slots count from 1.
pub fn hotbar_labels(items: &[&str]) -> Vec<String> {
    items
        .iter()
        .enumerate()
        .map(|(i, item)| format!("[{}] {item}", i + 1))
        .collect()
}

/// Counts [2, 3, 1] priced [10, 5, 100] is worth 2*10 + 3*5 + 1*100 = 135 emeralds.
pub fn loot_value(counts: &[u32], prices: &[u32]) -> u32 {
    counts.iter().zip(prices).map(|(c, p)| c * p).sum()
}

/// Pages start at 0. chest_page(&["a","b","c","d","e"], 1, 2) is ["c", "d"].
pub fn chest_page(items: &[&str], page: usize, size: usize) -> Vec<String> {
    items
        .iter()
        .skip(page * size)
        .take(size)
        .map(|s| s.to_string())
        .collect()
}

/// Tiers [1, 3, 2, 4] along the hotbar: how many times is the next slot a higher tier? -> 2
pub fn count_upgrades(tiers: &[u32]) -> usize {
    tiers.windows(2).filter(|pair| pair[1] > pair[0]).count()
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
