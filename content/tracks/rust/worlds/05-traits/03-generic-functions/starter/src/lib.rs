use std::fmt::Display;

/// The biggest item. `xs` is never empty.
pub fn largest<T: PartialOrd>(xs: &[T]) -> &T {
    // TODO: keep a reference to the best item so far, compare with `>`
    &xs[0]
}

/// "1, 2, 3" for [1, 2, 3].
pub fn show_all<T>(xs: &[T]) -> String
where
    T: Display,
{
    // TODO: turn each item into a String, then join with ", "
    todo!()
}

/// How many items equal `target`.
pub fn count_matching<T: PartialEq>(xs: &[T], target: &T) -> usize {
    // TODO: count the items equal to target
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_works_for_many_types() {
        assert_eq!(*largest(&[3, 9, 2]), 9);
        assert_eq!(*largest(&[1.5, 0.5, 4.25]), 4.25);
        assert_eq!(*largest(&["pear", "apple", "zebra"]), "zebra");
    }

    #[test]
    fn show_all_joins_with_commas() {
        assert_eq!(show_all(&[1, 2, 3]), "1, 2, 3");
        assert_eq!(show_all(&["a", "b"]), "a, b");
        assert_eq!(show_all::<i32>(&[]), "");
    }

    #[test]
    fn count_matching_counts_equal_items() {
        assert_eq!(count_matching(&[1, 2, 1, 3, 1], &1), 3);
        assert_eq!(count_matching(&["a", "b"], &"z"), 0);
    }
}
