use std::sync::Arc;
use std::thread;

/// One worker per jar; each worker reads its jar name from ONE shared list
/// (never a copy) and returns the name's length. Lengths come back in order.
pub fn name_lengths_on_workers(names: Vec<String>) -> Vec<usize> {
    let shared = Arc::new(names);
    let mut handles = Vec::new();
    for i in 0..shared.len() {
        let mine = Arc::clone(&shared);
        handles.push(thread::spawn(move || mine[i].len()));
    }
    let mut lengths = Vec::new();
    for handle in handles {
        lengths.push(handle.join().unwrap());
    }
    lengths
}

/// Another owner of the same list (no copy of the jar names).
pub fn share_jars(list: &Arc<Vec<String>>) -> Arc<Vec<String>> {
    Arc::clone(list)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn name_lengths_on_workers_read_one_shared_list() {
        let names = vec![String::from("sodium.jar"), String::from("jei.jar"), String::from("create.jar")];
        assert_eq!(name_lengths_on_workers(names), vec![10, 7, 10]);
    }

    #[test]
    fn share_hands_out_the_same_jar_list() {
        let list = Arc::new(vec![String::from("iris.jar")]);
        let other = share_jars(&list);
        assert!(Arc::ptr_eq(&list, &other), "both owners must point at the same jar list");
        assert_eq!(Arc::strong_count(&list), 2);
    }
}
