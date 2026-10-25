/// A temporary file. It writes "delete <name>" to the log when it is dropped.
/// It borrows the log, so it carries a lifetime `'a` like the borrowing structs of world 02.
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
        // TODO: push "delete <name>" onto self.log, with this file's name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn temp_file_is_deleted_at_the_brace() {
        let mut log = Vec::new();
        {
            let _file = TempFile::new("notes.tmp", &mut log);
        }
        assert_eq!(log, vec!["delete notes.tmp"]);
    }
}
