use rayon::prelude::*;

/// Sum of n * n over all numbers.
pub fn sum_of_squares(nums: &[u64]) -> u64 {
    // TODO: nums.par_iter().map(...).sum()
    todo!()
}

/// The length of every string, in the same order.
pub fn parallel_lengths(words: &[String]) -> Vec<usize> {
    // TODO: par_iter + map + collect
    todo!()
}

/// Given: trial division.
fn is_prime(n: u32) -> bool {
    n >= 2 && (2..).take_while(|d| d * d <= n).all(|d| !n.is_multiple_of(d))
}

/// How many primes are below `limit`?
pub fn count_primes(limit: u32) -> usize {
    // TODO: (0..limit).into_par_iter().filter(...).count()
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_of_squares_runs_in_parallel() {
        assert_eq!(sum_of_squares(&[1, 2, 3, 4]), 30);
        let big: Vec<u64> = (1..=1000).collect();
        assert_eq!(sum_of_squares(&big), 333_833_500);
        assert_eq!(sum_of_squares(&[]), 0);
    }

    #[test]
    fn parallel_lengths_keeps_order() {
        let words: Vec<String> = ["a", "abc", "", "hello"].iter().map(|s| s.to_string()).collect();
        assert_eq!(parallel_lengths(&words), vec![1, 3, 0, 5]);
    }

    #[test]
    fn count_primes_uses_into_par_iter() {
        assert_eq!(count_primes(0), 0);
        assert_eq!(count_primes(10), 4);
        assert_eq!(count_primes(100), 25);
        assert_eq!(count_primes(10_000), 1229);
    }
}
