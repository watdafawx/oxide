use std::fmt::Display;

pub struct Pair<T> {
    pub first: T,
    pub second: T,
}

impl<T> Pair<T> {
    pub fn new(first: T, second: T) -> Self {
        Pair { first, second }
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
    fn larger_picks_the_bigger_field() {
        assert_eq!(*Pair::new(3, 8).larger(), 8);
        assert_eq!(*Pair::new(2.5, 1.0).larger(), 2.5);
        assert_eq!(*Pair::new(4, 4).larger(), 4, "ties return first");
    }

    #[test]
    fn describe_larger_formats_text() {
        assert_eq!(Pair::new(3, 8).describe_larger(), "The larger is 8");
        assert_eq!(Pair::new("Serif", "Mono").describe_larger(), "The larger is Serif");
    }
}
