pub struct Countdown {
    n: u32,
}

impl Countdown {
    pub fn new(start: u32) -> Self {
        Countdown { n: start }
    }
}

impl Iterator for Countdown {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.n == 0 {
            None
        } else {
            let value = self.n;
            self.n -= 1;
            Some(value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_counts_down_then_stops() {
        let mut c = Countdown::new(3);
        assert_eq!(c.next(), Some(3));
        assert_eq!(c.next(), Some(2));
        assert_eq!(c.next(), Some(1));
        assert_eq!(c.next(), None, "0 is not said out loud");
        assert_eq!(Countdown::new(0).next(), None);
    }

    #[test]
    fn countdown_collects_like_a_range() {
        let v: Vec<u32> = Countdown::new(5).collect();
        assert_eq!(v, vec![5, 4, 3, 2, 1]);
    }
}
