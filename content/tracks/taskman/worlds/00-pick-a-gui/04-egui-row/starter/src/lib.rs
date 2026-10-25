//! The task manager's process row, drawn with egui.

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
        // TODO: a label for row.cpu, then a label for row.memory.
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui_kittest::Harness;
    use egui_kittest::kittest::Queryable;

    fn chrome() -> ProcessRow {
        ProcessRow::from_process(&Process { pid: 4120, name: "chrome.exe".to_string(), cpu: 12.46, memory_bytes: 536_870_912 })
    }

    #[test]
    fn row_shows_name_cpu_and_memory() {
        let row = chrome();
        let harness = Harness::new_ui(|ui| row_ui(ui, &row));
        harness.get_by_label("chrome.exe");
        harness.get_by_label("12.5%");
        harness.get_by_label("512 MB");
    }

    #[test]
    fn cells_sit_side_by_side() {
        let row = chrome();
        let harness = Harness::new_ui(|ui| row_ui(ui, &row));
        let name = harness.get_by_label("chrome.exe").rect();
        let cpu = harness.get_by_label("12.5%").rect();
        let memory = harness.get_by_label("512 MB").rect();
        assert!(cpu.left() >= name.right(), "12.5% should sit right of chrome.exe, not under it");
        assert!(memory.left() >= cpu.right(), "512 MB should sit right of 12.5%");
        assert!((cpu.top() - name.top()).abs() < 1.0 && (memory.top() - name.top()).abs() < 1.0, "all three should be on one line");
    }
}
