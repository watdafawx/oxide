pub struct Pair<T> {
    pub first: T,
    pub second: T,
}

impl<T> Pair<T> {
    pub fn new(first: T, second: T) -> Self {
        Pair { first, second }
    }

    /// Same values, places swapped. Uses up the old pair.
    pub fn swap(self) -> Pair<T> {
        Pair {
            first: self.second,
            second: self.first,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pair_holds_two_of_anything() {
        let sizes = Pair::new(12, 30);
        assert_eq!((sizes.first, sizes.second), (12, 30));
        let fonts = Pair::new("Mono", "Serif");
        assert_eq!((fonts.first, fonts.second), ("Mono", "Serif"));
    }

    #[test]
    fn swap_crosses_the_fields_over() {
        let p = Pair::new("left", "right").swap();
        assert_eq!((p.first, p.second), ("right", "left"));
        let q = Pair::new(String::from("a"), String::from("b")).swap();
        assert_eq!(q.first, "b", "works for owned Strings too, no clone needed");
    }
}
