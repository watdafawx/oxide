/// [1, 2, 3] -> [1, 4, 9]
pub fn squares(numbers: &[i32]) -> Vec<i32> {
    // TODO: square every number and collect them into a Vec
    todo!()
}

/// Keep only the even numbers: [1, 2, 3, 4] -> [2, 4]
pub fn evens(numbers: &[i32]) -> Vec<i32> {
    // TODO: keep only the even numbers (copy them out of the references)
    todo!()
}

/// Total number of bytes in all the words: ["ab", "cde"] -> 5
pub fn total_length(words: &[String]) -> usize {
    // TODO: add up the length of every word
    todo!()
}

/// How many words are at least `min_len` bytes long?
pub fn count_long(words: &[String], min_len: usize) -> usize {
    // TODO: count the words with at least `min_len` bytes
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn squares_maps_each_number() {
        assert_eq!(squares(&[1, 2, 3]), vec![1, 4, 9]);
        assert_eq!(squares(&[-4]), vec![16]);
        assert_eq!(squares(&[]), Vec::<i32>::new());
    }

    #[test]
    fn evens_filters() {
        assert_eq!(evens(&[1, 2, 3, 4, 5, 6]), vec![2, 4, 6]);
        assert_eq!(evens(&[1, 3]), Vec::<i32>::new());
    }

    #[test]
    fn total_length_sums_lengths() {
        assert_eq!(total_length(&strings(&["ab", "cde"])), 5);
        assert_eq!(total_length(&[]), 0);
    }

    #[test]
    fn count_long_counts_filtered() {
        let words = strings(&["a", "tree", "is", "green"]);
        assert_eq!(count_long(&words, 3), 2);
        assert_eq!(count_long(&words, 1), 4);
        assert_eq!(count_long(&words, 99), 0);
    }
}
