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
    fn inserter_logs_power_down() {
        let mut log = Vec::new();
        {
            let _i = Inserter::new("inserter-1", &mut log);
        }
        assert_eq!(log, vec!["power down inserter-1"]);
    }

    #[test]
    fn hot_swap_powers_down_first() {
        let mut log = Vec::new();
        hot_swap(&mut log);
        assert_eq!(log, vec!["power down inserter-1", "place new inserter"]);
    }
}
