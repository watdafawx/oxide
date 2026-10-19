use std::thread::{self, JoinHandle};

/// Starts a thread that computes n * n. Do not join here: return the handle.
pub fn spawn_square(n: u64) -> JoinHandle<u64> {
    // TODO: thread::spawn(move || ...) and return the handle
    todo!()
}

/// Squares every number on its own thread. Results are in the same order as `nums`.
pub fn square_all(nums: &[u64]) -> Vec<u64> {
    // TODO: spawn all threads first (collect the handles), then join each one
    todo!()
}

/// Computes a / b on another thread. If that thread panics, returns None.
pub fn safe_div(a: u32, b: u32) -> Option<u32> {
    // TODO: spawn a thread running `a / b`, join it, turn Err into None
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawn_square_runs_on_a_thread() {
        assert_eq!(spawn_square(7).join().unwrap(), 49);
    }

    #[test]
    fn square_all_keeps_input_order() {
        assert_eq!(square_all(&[3, 1, 4, 1, 5]), vec![9, 1, 16, 1, 25]);
        assert!(square_all(&[]).is_empty());
    }

    #[test]
    fn safe_div_turns_a_panic_into_none() {
        assert_eq!(safe_div(10, 2), Some(5));
        assert_eq!(safe_div(1, 0), None);
    }
}
