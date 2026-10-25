use std::fmt::Display;

/// The top item. `xs` is never empty; ties go to the first.
// TODO: compare items with `>`; when Rust asks, add the bound it names after `T`
pub fn top<T>(xs: &[T]) -> &T {
    &xs[0]
}

/// "top: 9" for kill counts [3, 9, 2].
pub fn announce_top<T>(xs: &[T]) -> String
where
    T: Display + PartialOrd,
{
    // TODO: return "top: <item>" using top
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn top_works_for_many_types() {
        assert_eq!(*top(&[3, 9, 2]), 9, "kill counts");
        assert_eq!(*top(&[1.5, 0.5, 4.25]), 4.25, "damage");
        assert_eq!(*top(&["Alex", "Steve", "Ari"]), "Steve", "names compare A to Z");
    }

    #[test]
    fn announce_top_prints_it() {
        assert_eq!(announce_top(&[3, 9, 2]), "top: 9");
        assert_eq!(announce_top(&["Alex", "Steve"]), "top: Steve");
    }
}
