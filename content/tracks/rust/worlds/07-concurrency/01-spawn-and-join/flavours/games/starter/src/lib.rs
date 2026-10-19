use std::thread::{self, JoinHandle};

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

/// Stand-in for a real hash: given, do not change.
pub fn quick_hash(jar: &ModJar) -> u64 {
    jar.bytes * 31 + jar.name.len() as u64
}

/// Hashes `jar` on its own thread. Do not join here: return the handle.
pub fn spawn_hash(jar: ModJar) -> JoinHandle<u64> {
    // TODO: thread::spawn(move || ...) and return the handle
    todo!()
}

/// Hashes every jar on its own thread. Hashes come back in the same order as `jars`.
pub fn hash_all(jars: Vec<ModJar>) -> Vec<u64> {
    // TODO: spawn all threads first (collect the handles), then join each one
    todo!()
}

/// Average jar size, computed on another thread. With zero jars the division
/// panics inside the thread, and the answer is None.
pub fn average_size(total_bytes: u32, jar_count: u32) -> Option<u32> {
    // TODO: spawn a thread running `total_bytes / jar_count`, join it, turn Err into None
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawn_hash_hashes_a_jar_on_a_thread() {
        let jar = ModJar::new("sodium.jar", 1000);
        assert_eq!(spawn_hash(jar).join().unwrap(), 31_010);
    }

    #[test]
    fn hash_all_keeps_the_mods_folder_order() {
        let jars = vec![
            ModJar::new("a.jar", 10),
            ModJar::new("lithium.jar", 20),
            ModJar::new("iris.jar", 30),
        ];
        assert_eq!(hash_all(jars), vec![315, 631, 938]);
        assert!(hash_all(Vec::new()).is_empty());
    }

    #[test]
    fn average_size_survives_an_empty_folder() {
        assert_eq!(average_size(900, 3), Some(300));
        assert_eq!(average_size(500, 0), None);
    }
}
