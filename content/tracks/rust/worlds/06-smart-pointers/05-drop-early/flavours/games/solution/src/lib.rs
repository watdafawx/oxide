/// An inserter. It writes "power down <name>" to the log when it is dropped.
pub struct Inserter<'a> {
    name: String,
    log: &'a mut Vec<String>,
}

impl<'a> Inserter<'a> {
    pub fn new(name: &str, log: &'a mut Vec<String>) -> Inserter<'a> {
        Inserter {
            name: name.to_string(),
            log,
        }
    }
}

impl Drop for Inserter<'_> {
    fn drop(&mut self) {
        self.log.push(format!("power down {}", self.name));
    }
}

/// Powers down "inserter-1", and only then logs "place new inserter".
pub fn hot_swap(log: &mut Vec<String>) {
    let old = Inserter::new("inserter-1", log);
    drop(old);
    log.push("place new inserter".to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hot_swap_powers_down_before_placing() {
        let mut log = Vec::new();
        hot_swap(&mut log);
        assert_eq!(log, vec!["power down inserter-1", "place new inserter"]);
    }
}
