use std::thread;

/// Builds "Hashing <jar_name>" on a new thread that owns the name.
pub fn label_in_thread(jar_name: String) -> String {
    // TODO: this does not build (E0373). Make the closure own jar_name.
    let handle = thread::spawn(|| format!("Hashing {jar_name}"));
    handle.join().unwrap()
}

/// Adds up the jar `sizes` on a new thread that owns the Vec.
pub fn total_in_thread(sizes: Vec<u64>) -> u64 {
    // TODO: move `sizes` into a thread, sum it there (sum::<u64>()), join and return the sum
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn label_in_thread_owns_the_jar_name() {
        assert_eq!(label_in_thread(String::from("sodium.jar")), "Hashing sodium.jar");
    }

    #[test]
    fn total_in_thread_takes_the_jar_sizes() {
        assert_eq!(total_in_thread(vec![100, 250, 650]), 1000);
        assert_eq!(total_in_thread(Vec::new()), 0);
    }
}
