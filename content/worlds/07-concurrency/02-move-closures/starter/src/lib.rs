use std::thread;

/// On a new thread, build "Hello, <name>!" and return it (the name is moved in).
pub fn greet_in_thread(name: String) -> String {
    // TODO: spawn(move || format!(...)), then join
    todo!()
}

/// Sums `nums` on a new thread that owns the Vec.
pub fn sum_in_thread(nums: Vec<i32>) -> i32 {
    // TODO: move `nums` into the closure, sum it there, join
    todo!()
}

/// Splits `data` at the middle (first half = len / 2 items), sums each half
/// on its own thread, returns (first half sum, second half sum).
pub fn split_work(mut data: Vec<u32>) -> (u32, u32) {
    // TODO: let right = data.split_off(mid); one thread per Vec; join both
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_in_thread_owns_the_name() {
        assert_eq!(greet_in_thread(String::from("Ferris")), "Hello, Ferris!");
    }

    #[test]
    fn sum_in_thread_takes_the_vec() {
        assert_eq!(sum_in_thread(vec![1, 2, 3, 4]), 10);
        assert_eq!(sum_in_thread(vec![]), 0);
    }

    #[test]
    fn split_work_sums_both_halves() {
        assert_eq!(split_work(vec![1, 2, 3, 4, 5, 6]), (6, 15));
        assert_eq!(split_work(vec![10, 20, 30]), (10, 50));
        assert_eq!(split_work(vec![]), (0, 0));
    }
}
