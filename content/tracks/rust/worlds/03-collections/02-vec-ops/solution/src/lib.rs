/// Remove the item at `index` and put it at the front. [a, b, c] with index 2 -> [c, a, b].
pub fn move_to_front(list: &mut Vec<String>, index: usize) {
    let item = list.remove(index);
    list.insert(0, item);
}

/// Sorted, with repeats removed. [3, 1, 3, 2, 1] -> [1, 2, 3].
pub fn sorted_unique(mut numbers: Vec<i32>) -> Vec<i32> {
    numbers.sort();
    numbers.dedup();
    numbers
}

/// Is the exact name "admin" in the list?
pub fn has_admin(names: &[String]) -> bool {
    names.contains(&"admin".to_string())
}

/// Keep only words that are at least `min_len` bytes long.
pub fn drop_short(words: &mut Vec<String>, min_len: usize) {
    words.retain(|w| w.len() >= min_len);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn move_to_front_removes_then_inserts() {
        let mut list = strings(&["a", "b", "c"]);
        move_to_front(&mut list, 2);
        assert_eq!(list, strings(&["c", "a", "b"]));
        move_to_front(&mut list, 0);
        assert_eq!(list, strings(&["c", "a", "b"]), "index 0 changes nothing");
    }

    #[test]
    fn sorted_unique_sorts_and_dedups() {
        assert_eq!(sorted_unique(vec![3, 1, 3, 2, 1]), vec![1, 2, 3]);
        assert_eq!(sorted_unique(vec![]), Vec::<i32>::new());
    }

    #[test]
    fn has_admin_uses_contains() {
        assert!(has_admin(&strings(&["bob", "admin"])));
        assert!(!has_admin(&strings(&["bob", "administrator"])));
    }

    #[test]
    fn drop_short_uses_retain() {
        let mut words = strings(&["a", "tree", "is", "green"]);
        drop_short(&mut words, 3);
        assert_eq!(words, strings(&["tree", "green"]));
    }
}
