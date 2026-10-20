use std::rc::Rc;

/// Returns another owner of the same String (no copy of the text).
pub fn share(shared: &Rc<String>) -> Rc<String> {
    // TODO: hand out a second owner of the same value, not a copy of it
    todo!()
}

/// How many owners does this Rc have right now?
pub fn owner_count(shared: &Rc<String>) -> usize {
    // TODO: how many owners does this value have right now?
    todo!()
}

/// Returns (count while a temporary extra owner exists, count after it is gone).
pub fn count_inside_and_after(shared: &Rc<String>) -> (usize, usize) {
    // TODO: in an inner { } block make an extra owner and read its count;
    // after the block, read the count again
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn share_returns_another_handle() {
        let a = Rc::new(String::from("song"));
        let b = share(&a);
        assert!(Rc::ptr_eq(&a, &b), "both handles must point at the same String");
        assert_eq!(Rc::strong_count(&a), 2);
    }

    #[test]
    fn owner_count_reports_strong_count() {
        let a = Rc::new(String::from("song"));
        let _b = Rc::clone(&a);
        let _c = Rc::clone(&a);
        assert_eq!(owner_count(&a), 3);
    }

    #[test]
    fn count_inside_and_after_tracks_drops() {
        let a = Rc::new(String::from("song"));
        assert_eq!(count_inside_and_after(&a), (2, 1));
    }
}
