/// An inserter. It writes "power down <name>" to the log when it is dropped.
/// It borrows the log, so it carries a lifetime `'a` like the borrowing structs of world 02.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inserter_powers_down_at_the_brace() {
        let mut log = Vec::new();
        {
            let _inserter = Inserter::new("inserter-1", &mut log);
        }
        assert_eq!(log, vec!["power down inserter-1"]);
    }
}
