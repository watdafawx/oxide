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
        // TODO: return None at 0, otherwise return the current n and count down by one
        todo!()
    }
}

/// The even numbers from `start` down to 1.
pub fn evens(start: u32) -> Vec<u32> {
    // TODO: filter a Countdown to even numbers and collect
    Vec::new()
}

/// 3 * 3 + 2 * 2 + 1 * 1 for start = 3.
pub fn sum_of_squares(start: u32) -> u32 {
    // TODO: map each number to its square and sum
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_down_to_one() {
        let v: Vec<u32> = Countdown::new(3).collect();
        assert_eq!(v, vec![3, 2, 1]);
        assert_eq!(Countdown::new(0).next(), None, "0 yields nothing");
    }

    #[test]
    fn evens_only() {
        assert_eq!(evens(6), vec![6, 4, 2]);
        assert_eq!(evens(1), Vec::<u32>::new());
    }

    #[test]
    fn sum_of_squares_of_countdown() {
        assert_eq!(sum_of_squares(3), 14);
        assert_eq!(sum_of_squares(0), 0);
    }
}
