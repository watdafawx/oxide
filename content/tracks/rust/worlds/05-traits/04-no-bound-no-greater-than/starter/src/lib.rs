use std::fmt::Display;

/// The biggest item. `xs` is never empty; ties go to the first.
// TODO: compare items with `>`; when Rust asks, add the bound it names after `T`
pub fn largest<T>(xs: &[T]) -> &T {
    &xs[0]
}

/// "largest: 9" for [3, 9, 2].
pub fn announce_largest<T>(xs: &[T]) -> String
where
    T: Display + PartialOrd,
{
    // TODO: return "largest: <item>" using largest
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_works_for_many_types() {
        assert_eq!(*largest(&[3, 9, 2]), 9, "pixels");
        assert_eq!(*largest(&[1.5, 0.5, 4.25]), 4.25, "vector sizes");
        assert_eq!(*largest(&["Mono", "Arial", "Serif"]), "Serif", "names compare A to Z");
    }

    #[test]
    fn announce_largest_prints_it() {
        assert_eq!(announce_largest(&[3, 9, 2]), "largest: 9");
        assert_eq!(announce_largest(&["Mono", "Arial"]), "largest: Mono");
    }
}
