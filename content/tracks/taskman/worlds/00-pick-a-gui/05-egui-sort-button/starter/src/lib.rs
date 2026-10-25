//! The task manager's process list in egui, with a CPU button that sorts it.

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

/// Draws one row: name, CPU and memory, left to right.
pub fn row_ui(ui: &mut egui::Ui, row: &ProcessRow) {
    ui.horizontal(|ui| {
        ui.label(row.name.as_str());
        ui.label(row.cpu.as_str());
        ui.label(row.memory.as_str());
    });
}

/// The whole list.
pub struct TaskList {
    pub processes: Vec<Process>,
}

impl TaskList {
    /// Highest CPU first.
    pub fn sort_by_cpu(&mut self) {
        self.processes.sort_by(|a, b| b.cpu.total_cmp(&a.cpu));
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        // TODO: a "CPU" button; when it is clicked, call self.sort_by_cpu().
        for process in &self.processes {
            row_ui(ui, &ProcessRow::from_process(process));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui_kittest::Harness;
    use egui_kittest::kittest::Queryable;

    fn process(name: &str, cpu: f32) -> Process {
        Process { pid: 1, name: name.to_string(), cpu, memory_bytes: 100 << 20 }
    }

    fn names(list: &TaskList) -> Vec<&str> {
        list.processes.iter().map(|p| p.name.as_str()).collect()
    }

    #[test]
    fn clicking_cpu_puts_busiest_first() {
        let list = TaskList { processes: vec![process("explorer.exe", 0.5), process("chrome.exe", 12.5), process("game.exe", 40.0)] };
        let mut harness = Harness::new_ui_state(|ui, list: &mut TaskList| list.ui(ui), list);
        harness.get_by_label("CPU").click();
        harness.run();
        assert_eq!(names(harness.state()), ["game.exe", "chrome.exe", "explorer.exe"]);
        let game = harness.get_by_label("game.exe").rect();
        let chrome = harness.get_by_label("chrome.exe").rect();
        assert!(game.top() < chrome.top(), "game.exe should be drawn above chrome.exe after the click");
    }
}
