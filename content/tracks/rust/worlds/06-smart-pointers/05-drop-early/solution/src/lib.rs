/// A temporary file. It writes "delete <name>" to the log when it is dropped.
pub struct TempFile<'a> {
    name: String,
    log: &'a mut Vec<String>,
}

impl<'a> TempFile<'a> {
    pub fn new(name: &str, log: &'a mut Vec<String>) -> TempFile<'a> {
        TempFile {
            name: name.to_string(),
            log,
        }
    }
}

impl Drop for TempFile<'_> {
    fn drop(&mut self) {
        self.log.push(format!("delete {}", self.name));
    }
}

/// Writes "save.tmp", deletes it, and only then logs "saved".
pub fn save(log: &mut Vec<String>) {
    let file = TempFile::new("save.tmp", log);
    drop(file);
    log.push("saved".to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_deletes_temp_file_before_logging_saved() {
        let mut log = Vec::new();
        save(&mut log);
        assert_eq!(log, vec!["delete save.tmp", "saved"]);
    }
}
