use std::thread;

/// On a new thread, build "Hello, <name>!" and return it (the name is moved in).
pub fn greet_in_thread(name: String) -> String {
    thread::spawn(move || format!("Hello, {name}!")).join().unwrap()
}

/// Sums `nums` on a new thread that owns the Vec.
pub fn sum_in_thread(nums: Vec<i32>) -> i32 {
    thread::spawn(move || nums.iter().sum::<i32>()).join().unwrap()
}

/// Splits `data` at the middle (first half = len / 2 items), sums each half
/// on its own thread, returns (first half sum, second half sum).
pub fn split_work(mut data: Vec<u32>) -> (u32, u32) {
    let mid = data.len() / 2;
    let right = data.split_off(mid);
    let left_handle = thread::spawn(move || data.iter().sum::<u32>());
    let right_handle = thread::spawn(move || right.iter().sum::<u32>());
    (left_handle.join().unwrap(), right_handle.join().unwrap())
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
