use std::thread;

/// Average file size, computed on another thread. With zero files the division
/// panics inside that thread, and the answer is None.
pub fn average_size(total_bytes: u64, file_count: u64) -> Option<u64> {
    thread::spawn(move || total_bytes / file_count).join().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn average_size_survives_an_empty_folder() {
        assert_eq!(average_size(900, 3), Some(300));
        assert_eq!(average_size(500, 0), None, "zero files: the thread panics, so the answer is None");
    }
}
