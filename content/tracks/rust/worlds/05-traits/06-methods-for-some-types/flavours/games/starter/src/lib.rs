use std::fmt::Display;

pub struct Hands<T> {
    pub main_hand: T,
    pub off_hand: T,
}

impl<T> Hands<T> {
    pub fn new(main_hand: T, off_hand: T) -> Self {
        Hands { main_hand, off_hand }
    }
}

impl<T: PartialOrd> Hands<T> {
    /// The better of the two. Ties return `main_hand`.
    pub fn best(&self) -> &T {
        // TODO: compare the items and return a reference to the better one
        &self.off_hand
    }
}

impl<T: Display + PartialOrd> Hands<T> {
    pub fn describe_best(&self) -> String {
        // TODO: return "Best item: <item>" using self.best()
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn best_picks_the_better_item() {
        assert_eq!(*Hands::new(16, 64).best(), 64);
        assert_eq!(*Hands::new(7.5, 4.0).best(), 7.5, "attack damage");
        assert_eq!(*Hands::new(32, 32).best(), 32, "ties return main_hand");
    }

    #[test]
    fn describe_best_formats_text() {
        assert_eq!(Hands::new(16, 64).describe_best(), "Best item: 64");
        assert_eq!(Hands::new("torch", "dirt").describe_best(), "Best item: torch");
    }
}
