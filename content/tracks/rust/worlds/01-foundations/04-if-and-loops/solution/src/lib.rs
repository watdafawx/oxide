/// 3 -> "Fizz", 5 -> "Buzz", 15 -> "FizzBuzz", 7 -> "7"
pub fn fizzbuzz(n: i32) -> String {
    if n % 15 == 0 {
        "FizzBuzz".to_string()
    } else if n % 3 == 0 {
        "Fizz".to_string()
    } else if n % 5 == 0 {
        "Buzz".to_string()
    } else {
        n.to_string()
    }
}

/// sum_to(4) -> 1 + 2 + 3 + 4 = 10
pub fn sum_to(n: u32) -> u32 {
    let mut total = 0;
    for i in 1..=n {
        total += i;
    }
    total
}

/// Steps until n reaches 1: even -> n / 2, odd -> 3n + 1.
/// collatz_steps(6) -> 8 (6, 3, 10, 5, 16, 8, 4, 2, 1)
pub fn collatz_steps(mut n: i64) -> u32 {
    let mut steps = 0;
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        steps += 1;
    }
    steps
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
