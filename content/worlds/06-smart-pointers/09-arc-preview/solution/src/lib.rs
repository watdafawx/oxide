use std::sync::Arc;
use std::thread;

/// Sums `data` using two threads that share it through an Arc.
pub fn parallel_sum(data: Vec<i32>) -> i32 {
    let data = Arc::new(data);
    let mid = data.len() / 2;
    let first = Arc::clone(&data);
    let left = thread::spawn(move || first[..mid].iter().sum::<i32>());
    let second = Arc::clone(&data);
    let right = thread::spawn(move || second[mid..].iter().sum::<i32>());
    left.join().unwrap() + right.join().unwrap()
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
