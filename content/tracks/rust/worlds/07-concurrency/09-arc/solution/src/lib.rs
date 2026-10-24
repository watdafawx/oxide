use std::sync::Arc;
use std::thread;

/// One thread per name; each thread reads its name from ONE shared list
/// (never a copy) and returns the name's length. Lengths come back in order.
pub fn lengths_on_threads(names: Vec<String>) -> Vec<usize> {
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

/// Another owner of the same list (no copy of the names).
pub fn share(list: &Arc<Vec<String>>) -> Arc<Vec<String>> {
    Arc::clone(list)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn lengths_on_threads_read_one_shared_list() {
        let names = vec![String::from("photo.jpg"), String::from("a.txt"), String::from("report.pdf")];
        assert_eq!(lengths_on_threads(names), vec![9, 5, 10]);
    }

    #[test]
    fn share_hands_out_the_same_list() {
        let list = Arc::new(vec![String::from("a.txt")]);
        let other = share(&list);
        assert!(Arc::ptr_eq(&list, &other), "both owners must point at the same list");
        assert_eq!(Arc::strong_count(&list), 2);
    }
}
