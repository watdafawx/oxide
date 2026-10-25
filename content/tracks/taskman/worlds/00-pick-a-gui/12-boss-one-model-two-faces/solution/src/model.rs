//! What the list holds and what it shows. No GUI library in here.

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
            cpu: format!("{:.1}%", process.cpu),
            memory: format!("{} MB", process.memory_bytes / 1_048_576),
        }
    }
}

/// Which column the list is sorted by.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortBy {
    Cpu,
    Memory,
    Name,
}

/// The whole list: every process, the sort column and the search text.
pub struct TaskList {
    pub processes: Vec<Process>,
    pub sort: SortBy,
    pub filter: String,
}

impl TaskList {
    pub fn new(processes: Vec<Process>) -> TaskList {
        TaskList { processes, sort: SortBy::Cpu, filter: String::new() }
    }

    /// The rows to draw: names that contain the filter (in any case), in sort order.
    pub fn visible(&self) -> Vec<ProcessRow> {
        let needle = self.filter.to_lowercase();
        let mut shown: Vec<&Process> =
            self.processes.iter().filter(|p| p.name.to_lowercase().contains(&needle)).collect();
        match self.sort {
            SortBy::Cpu => shown.sort_by(|a, b| b.cpu.total_cmp(&a.cpu)),
            SortBy::Memory => shown.sort_by_key(|p| std::cmp::Reverse(p.memory_bytes)),
            SortBy::Name => shown.sort_by_key(|p| p.name.to_lowercase()),
        }
        shown.into_iter().map(ProcessRow::from_process).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sample_processes;

    fn names(list: &TaskList) -> Vec<String> {
        list.visible().into_iter().map(|row| row.name).collect()
    }

    #[test]
    fn memory_sort_is_biggest_first() {
        let mut list = TaskList::new(sample_processes());
        list.sort = SortBy::Memory;
        assert_eq!(names(&list), ["game.exe", "chrome.exe", "Zoom.exe", "explorer.exe", "Chrome Helper.exe"]);
    }

    #[test]
    fn name_sort_ignores_case() {
        let mut list = TaskList::new(sample_processes());
        list.sort = SortBy::Name;
        assert_eq!(
            names(&list),
            ["Chrome Helper.exe", "chrome.exe", "explorer.exe", "game.exe", "Zoom.exe"],
            "Zoom.exe sorts with the z's, not before every lowercase name"
        );
    }

    #[test]
    fn filter_matches_any_case() {
        let mut list = TaskList::new(sample_processes());
        list.filter = "CHROME".to_string();
        assert_eq!(names(&list), ["chrome.exe", "Chrome Helper.exe"], "sorted by CPU, the default");
        list.filter = String::new();
        assert_eq!(names(&list).len(), 5, "an empty filter shows everything");
    }
}
