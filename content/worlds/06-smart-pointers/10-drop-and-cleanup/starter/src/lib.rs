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
