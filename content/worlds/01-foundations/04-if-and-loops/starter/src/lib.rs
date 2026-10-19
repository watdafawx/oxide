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
}
