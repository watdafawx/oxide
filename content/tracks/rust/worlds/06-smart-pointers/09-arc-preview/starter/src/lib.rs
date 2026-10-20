use std::sync::Arc;
use std::thread;

/// Sums `data` using two threads that share it through an Arc.
pub fn parallel_sum(data: Vec<i32>) -> i32 {
    // TODO: split the data in half, sum each half on its own thread (give each thread its own Arc owner),
    // then wait for both and add the two sums
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parallel_sum_adds_everything() {
        let data: Vec<i32> = (1..=101).collect();
        assert_eq!(parallel_sum(data), 5151);
    }

    #[test]
    fn parallel_sum_handles_tiny_inputs() {
        assert_eq!(parallel_sum(Vec::new()), 0, "empty input");
        assert_eq!(parallel_sum(vec![7]), 7, "one element");
    }
}
