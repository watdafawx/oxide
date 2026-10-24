use std::thread;

/// Average jar size, computed on a worker thread. With zero jars the division
/// panics inside that worker, and the answer is None.
pub fn average_jar_size(total_bytes: u64, jar_count: u64) -> Option<u64> {
    thread::spawn(move || total_bytes / jar_count).join().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn average_jar_size_survives_an_empty_mods_folder() {
        assert_eq!(average_jar_size(900, 3), Some(300));
        assert_eq!(average_jar_size(500, 0), None, "zero jars: the worker panics, so the answer is None");
    }
}
