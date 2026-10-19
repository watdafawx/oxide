/// 3 -> "Fizz", 5 -> "Buzz", 15 -> "FizzBuzz", 7 -> "7"
pub fn fizzbuzz(n: i32) -> String {
    // TODO: if / else if / else chain (check multiples of 15 first)
    todo!()
}

/// sum_to(4) -> 1 + 2 + 3 + 4 = 10
pub fn sum_to(n: u32) -> u32 {
    // TODO: for i in 1..=n { ... }
    todo!()
}

/// Steps until n reaches 1: even -> n / 2, odd -> 3n + 1.
/// collatz_steps(6) -> 8 (6, 3, 10, 5, 16, 8, 4, 2, 1)
pub fn collatz_steps(mut n: i64) -> u32 {
    // TODO: count steps in a `while n != 1` loop
    todo!()
}

/// Smallest n (starting at 1) whose square is bigger than `limit`.
pub fn first_square_over(limit: u32) -> u32 {
    // TODO: `loop` with `break n;`
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fizzbuzz_words() {
        assert_eq!(fizzbuzz(3), "Fizz");
        assert_eq!(fizzbuzz(10), "Buzz");
        assert_eq!(fizzbuzz(30), "FizzBuzz");
        assert_eq!(fizzbuzz(7), "7");
    }

    #[test]
    fn sum_to_adds_range() {
        assert_eq!(sum_to(4), 10);
        assert_eq!(sum_to(0), 0, "an empty range adds nothing");
    }

    #[test]
    fn collatz_counts_steps() {
        assert_eq!(collatz_steps(6), 8);
        assert_eq!(collatz_steps(1), 0, "already at 1");
    }

    #[test]
    fn first_square_over_breaks_with_value() {
        assert_eq!(first_square_over(50), 8, "8 * 8 = 64 is the first square over 50");
        assert_eq!(first_square_over(0), 1);
    }
}
