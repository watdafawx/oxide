/// The first number bigger than `limit`.
pub fn first_over(numbers: &[i32], limit: i32) -> Option<i32> {
    // TODO: numbers.iter().copied().find(|n| ...)
    todo!()
}

/// Where in the list is `name`? None if it is not there.
pub fn index_of(names: &[String], name: &str) -> Option<usize> {
    // TODO: names.iter().position(|n| ...): compare n.as_str() == name
    todo!()
}

/// Is any number below zero?
pub fn has_negative(numbers: &[i32]) -> bool {
    // TODO: numbers.iter().any(|n| ...)
    todo!()
}

/// Does every score reach `pass`? An empty list counts as yes.
pub fn all_passing(scores: &[u32], pass: u32) -> bool {
    // TODO: scores.iter().all(|s| ...)
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn find_and_position_search() {
        assert_eq!(first_over(&[4, 8, 15, 16], 10), Some(15));
        assert_eq!(first_over(&[4, 8], 10), None);
        let names = strings(&["ann", "bo", "cy"]);
        assert_eq!(index_of(&names, "bo"), Some(1));
        assert_eq!(index_of(&names, "zed"), None);
    }

    #[test]
    fn any_and_all_check() {
        assert!(has_negative(&[3, -1, 2]));
        assert!(!has_negative(&[3, 0, 2]));
        assert!(all_passing(&[60, 75, 90], 60));
        assert!(!all_passing(&[60, 59], 60));
        assert!(all_passing(&[], 60), "no scores, nothing failing");
    }
}
