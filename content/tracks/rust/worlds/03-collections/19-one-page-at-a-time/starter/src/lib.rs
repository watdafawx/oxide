/// The first `n` names: ([pen, mug, lamp], 2) -> [pen, mug]
pub fn first_lines(names: &[String], n: usize) -> Vec<String> {
    // TODO: keep the first `n` names and collect copies of them
    Vec::new()
}

/// Page `page` (from 0) with `size` names per page: ([a, b, c, d, e], 1, 2) -> [c, d]
pub fn page(names: &[String], page: usize, size: usize) -> Vec<String> {
    // TODO: pass over the earlier pages, take one page, collect copies
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn first_lines_takes_n() {
        let list = names(&["pen", "mug", "lamp"]);
        assert_eq!(first_lines(&list, 2), names(&["pen", "mug"]));
        assert_eq!(first_lines(&list, 9), list, "fewer than n is fine");
    }

    #[test]
    fn page_skips_then_takes() {
        let list = names(&["a", "b", "c", "d", "e"]);
        assert_eq!(page(&list, 0, 2), names(&["a", "b"]));
        assert_eq!(page(&list, 1, 2), names(&["c", "d"]));
        assert_eq!(page(&list, 2, 2), names(&["e"]), "the last page can be short");
        assert_eq!(page(&list, 9, 2), Vec::<String>::new());
    }
}
