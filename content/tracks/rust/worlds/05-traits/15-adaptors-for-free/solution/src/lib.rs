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

/// The even numbers from `start` down to 1.
pub fn evens(start: u32) -> Vec<u32> {
    Countdown::new(start).filter(|n| n % 2 == 0).collect()
}

/// 3 * 3 + 2 * 2 + 1 * 1 for start = 3.
pub fn sum_of_squares(start: u32) -> u32 {
    Countdown::new(start).map(|n| n * n).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evens_keeps_even_numbers() {
        assert_eq!(evens(6), vec![6, 4, 2]);
        assert_eq!(evens(1), Vec::<u32>::new());
    }

    #[test]
    fn sum_of_squares_of_countdown() {
        assert_eq!(sum_of_squares(3), 14);
        assert_eq!(sum_of_squares(0), 0);
    }
}
