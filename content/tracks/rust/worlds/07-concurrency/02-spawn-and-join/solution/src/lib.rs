use std::thread::{self, JoinHandle};

/// Stand-in for reading a file and hashing it: given, do not change.
pub fn checksum(file_name: &str) -> u64 {
    file_name.bytes().map(u64::from).sum::<u64>() * 31
}

/// Starts hashing "report.pdf" on a new thread. Does not wait: returns the handle.
pub fn start_check() -> JoinHandle<u64> {
    thread::spawn(|| checksum("report.pdf"))
}

/// Starts the check, waits for it to finish, and returns its answer.
pub fn check_and_wait() -> u64 {
    start_check().join().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_check_hashes_on_a_thread() {
        let handle = start_check();
        assert_eq!(handle.join().unwrap(), checksum("report.pdf"));
    }

    #[test]
    fn check_and_wait_returns_the_answer() {
        assert_eq!(check_and_wait(), checksum("report.pdf"));
    }
}
