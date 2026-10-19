use std::thread;

/// One jar from the mods folder.
#[derive(Debug, Clone)]
pub struct ModJar {
    pub name: String,
    pub bytes: u64,
}

impl ModJar {
    pub fn new(name: &str, bytes: u64) -> ModJar {
        ModJar {
            name: name.to_string(),
            bytes,
        }
    }
}

/// On a new thread, build "Hashing <jar_name>" and return it (the name is moved in).
pub fn label_in_thread(jar_name: String) -> String {
    thread::spawn(move || format!("Hashing {jar_name}")).join().unwrap()
}

/// Adds up `sizes` on a new thread that owns the Vec.
pub fn total_bytes_in_thread(sizes: Vec<u64>) -> u64 {
    thread::spawn(move || sizes.iter().sum::<u64>()).join().unwrap()
}

/// Splits `jars` at the middle (first half = len / 2 jars), totals the bytes of
/// each half on its own thread, returns (first half total, second half total).
pub fn split_folder(mut jars: Vec<ModJar>) -> (u64, u64) {
    let mid = jars.len() / 2;
    let right = jars.split_off(mid);
    let left_handle = thread::spawn(move || jars.iter().map(|j| j.bytes).sum::<u64>());
    let right_handle = thread::spawn(move || right.iter().map(|j| j.bytes).sum::<u64>());
    (left_handle.join().unwrap(), right_handle.join().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn label_in_thread_owns_the_jar_name() {
        assert_eq!(label_in_thread(String::from("sodium.jar")), "Hashing sodium.jar");
    }

    #[test]
    fn total_bytes_in_thread_takes_the_sizes() {
        assert_eq!(total_bytes_in_thread(vec![100, 250, 650]), 1000);
        assert_eq!(total_bytes_in_thread(vec![]), 0);
    }

    #[test]
    fn split_folder_totals_both_halves() {
        let jars = vec![
            ModJar::new("sodium.jar", 1),
            ModJar::new("lithium.jar", 2),
            ModJar::new("iris.jar", 3),
            ModJar::new("jei.jar", 4),
            ModJar::new("ae2.jar", 5),
            ModJar::new("create.jar", 6),
        ];
        assert_eq!(split_folder(jars), (6, 15));
        let three = vec![
            ModJar::new("a.jar", 10),
            ModJar::new("b.jar", 20),
            ModJar::new("c.jar", 30),
        ];
        assert_eq!(split_folder(three), (10, 50));
        assert_eq!(split_folder(Vec::new()), (0, 0));
    }
}
