use std::ops::Deref;

/// A handle to a machine. It acts like the machine inside.
pub struct InserterHandle<T>(pub T);

impl<T> InserterHandle<T> {
    pub fn new(machine: T) -> InserterHandle<T> {
        InserterHandle(machine)
    }
}

impl<T> Deref for InserterHandle<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

/// Takes a plain &str, so passing &InserterHandle<String> relies on deref coercion.
pub fn label_len(label: &str) -> usize {
    label.len()
}

/// Writes "power down <name>" to the log when it goes out of scope.
pub struct Inserter<'a> {
    name: &'static str,
    log: &'a mut Vec<String>,
}

impl<'a> Inserter<'a> {
    pub fn new(name: &'static str, log: &'a mut Vec<String>) -> Inserter<'a> {
        Inserter { name, log }
    }
}

impl Drop for Inserter<'_> {
    fn drop(&mut self) {
        self.log.push(format!("power down {}", self.name));
    }
}

/// Places "inserter-1", powers it down, and only then logs "place new inserter".
pub fn hot_swap(log: &mut Vec<String>) {
    let i = Inserter::new("inserter-1", log);
    drop(i);
    log.push("place new inserter".to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deref_gives_inner_value() {
        let b = InserterHandle::new(40);
        assert_eq!(*b, 40);
        let s = InserterHandle::new(String::from("fast-inserter"));
        assert_eq!(label_len(&s), 13, "&InserterHandle<String> should coerce to &str");
    }

    #[test]
    fn drop_logs_name() {
        let mut log = Vec::new();
        {
            let _i = Inserter::new("inserter-1", &mut log);
        }
        assert_eq!(log, vec!["power down inserter-1"]);
    }

    #[test]
    fn early_release_drops_before_work() {
        let mut log = Vec::new();
        hot_swap(&mut log);
        assert_eq!(log, vec!["power down inserter-1", "place new inserter"]);
    }
}
