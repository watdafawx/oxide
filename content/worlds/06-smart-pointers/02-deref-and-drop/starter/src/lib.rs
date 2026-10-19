use std::ops::Deref;

/// A box-like wrapper around any value.
pub struct MyBox<T>(pub T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        // TODO: return a reference to the wrapped value (field 0)
        todo!()
    }
}

/// Takes a plain &str, so passing &MyBox<String> relies on deref coercion.
pub fn str_len(s: &str) -> usize {
    s.len()
}

/// Writes "drop <name>" to the log when it goes out of scope.
pub struct Guard<'a> {
    name: &'static str,
    log: &'a mut Vec<String>,
}

impl<'a> Guard<'a> {
    pub fn new(name: &'static str, log: &'a mut Vec<String>) -> Guard<'a> {
        Guard { name, log }
    }
}

impl Drop for Guard<'_> {
    fn drop(&mut self) {
        // TODO: push format!("drop {}", self.name) onto self.log
    }
}

/// Creates Guard "a", releases it, and only then logs "work".
pub fn early_release(log: &mut Vec<String>) {
    // TODO: let g = Guard::new("a", log); drop(g); then log.push("work".to_string())
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deref_gives_inner_value() {
        let b = MyBox::new(5);
        assert_eq!(*b, 5);
        let s = MyBox::new(String::from("hi"));
        assert_eq!(str_len(&s), 2, "&MyBox<String> should coerce to &str");
    }

    #[test]
    fn drop_logs_name() {
        let mut log = Vec::new();
        {
            let _g = Guard::new("a", &mut log);
        }
        assert_eq!(log, vec!["drop a"]);
    }

    #[test]
    fn early_release_drops_before_work() {
        let mut log = Vec::new();
        early_release(&mut log);
        assert_eq!(log, vec!["drop a", "work"]);
    }
}
