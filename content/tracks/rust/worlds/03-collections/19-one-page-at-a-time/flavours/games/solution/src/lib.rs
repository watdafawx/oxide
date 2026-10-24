/// The first `n` items: ([sword, bow, torch], 2) -> [sword, bow]
pub fn first_items(items: &[String], n: usize) -> Vec<String> {
    items.iter().take(n).map(|item| item.clone()).collect()
}

/// Page `page` (from 0) with `size` items per page: ([a, b, c, d, e], 1, 2) -> [c, d]
pub fn chest_page(items: &[String], page: usize, size: usize) -> Vec<String> {
    items.iter().skip(page * size).take(size).map(|item| item.clone()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn first_items_takes_n() {
        let list = names(&["sword", "bow", "torch"]);
        assert_eq!(first_items(&list, 2), names(&["sword", "bow"]));
        assert_eq!(first_items(&list, 9), list, "fewer than n is fine");
    }

    #[test]
    fn chest_page_skips_then_takes() {
        let list = names(&["a", "b", "c", "d", "e"]);
        assert_eq!(chest_page(&list, 0, 2), names(&["a", "b"]));
        assert_eq!(chest_page(&list, 1, 2), names(&["c", "d"]));
        assert_eq!(chest_page(&list, 2, 2), names(&["e"]), "the last page can be short");
        assert_eq!(chest_page(&list, 9, 2), Vec::<String>::new());
    }
}
