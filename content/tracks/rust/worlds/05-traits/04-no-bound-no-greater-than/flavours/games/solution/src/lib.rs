use std::fmt::Display;

/// The top item. `xs` is never empty; ties go to the first.
pub fn top<T: PartialOrd>(xs: &[T]) -> &T {
    let mut best = &xs[0];
    for x in xs {
        if x > best {
            best = x;
        }
    }
    best
}

/// "top: 9" for kill counts [3, 9, 2].
pub fn announce_top<T>(xs: &[T]) -> String
where
    T: Display + PartialOrd,
{
    format!("top: {}", top(xs))
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
