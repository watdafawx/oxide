//! The task manager's first piece: what one row of the process list says.

/// One process, as Windows reports it.
pub struct Process {
    pub pid: u32,
    pub name: String,
    /// Percent of the whole CPU, e.g. 12.46.
    pub cpu: f32,
    pub memory_bytes: u64,
}

/// The three texts one row of the list shows.
#[derive(Debug, PartialEq)]
pub struct ProcessRow {
    pub name: String,
    pub cpu: String,
    pub memory: String,
}

impl ProcessRow {
    pub fn from_process(process: &Process) -> ProcessRow {
        ProcessRow {
            name: process.name.clone(),
            // TODO: one decimal, then a % sign: 12.46 becomes "12.5%".
            cpu: String::new(),
            // TODO: whole megabytes (bytes divided by 1_048_576), then " MB".
            memory: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chrome() -> Process {
        Process { pid: 4120, name: "chrome.exe".to_string(), cpu: 12.46, memory_bytes: 536_870_912 }
    }

    #[test]
    fn cpu_text_has_one_decimal() {
        let row = ProcessRow::from_process(&chrome());
        assert_eq!(row.name, "chrome.exe");
        assert_eq!(row.cpu, "12.5%");
        let idle = Process { cpu: 0.0, ..chrome() };
        assert_eq!(ProcessRow::from_process(&idle).cpu, "0.0%");
    }

    #[test]
    fn memory_text_is_whole_megabytes() {
        assert_eq!(ProcessRow::from_process(&chrome()).memory, "512 MB");
        let small = Process { memory_bytes: 3_000_000, ..chrome() };
        assert_eq!(ProcessRow::from_process(&small).memory, "2 MB", "2.86 MB rounds down to 2");
    }
}
