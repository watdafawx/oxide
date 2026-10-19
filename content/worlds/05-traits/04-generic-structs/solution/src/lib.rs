use std::fmt::Display;

pub struct Pair<T> {
    pub first: T,
    pub second: T,
}

impl<T> Pair<T> {
    pub fn new(first: T, second: T) -> Self {
        Pair { first, second }
    }

    /// Same values, swapped places.
    pub fn swap(self) -> Pair<T> {
        Pair {
            first: self.second,
            second: self.first,
        }
    }
}

impl<T: PartialOrd> Pair<T> {
    /// The bigger of the two. Ties return `first`.
    pub fn larger(&self) -> &T {
        if self.first >= self.second {
            &self.first
        } else {
            &self.second
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    pub fn describe_larger(&self) -> String {
        format!("The larger is {}", self.larger())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_and_swap() {
        let p = Pair::new("a", "b").swap();
        assert_eq!((p.first, p.second), ("b", "a"));
        let q = Pair::new(1, 2);
        assert_eq!((q.first, q.second), (1, 2));
    }

    #[test]
    fn larger_picks_bigger() {
        assert_eq!(*Pair::new(3, 8).larger(), 8);
        assert_eq!(*Pair::new(2.5, 1.0).larger(), 2.5);
    }

    #[test]
    fn describe_larger_formats_text() {
        assert_eq!(Pair::new(3, 8).describe_larger(), "The larger is 8");
        assert_eq!(Pair::new("pear", "apple").describe_larger(), "The larger is pear");
    }
}
