//! The task manager's process row, drawn with GPUI.

use gpui::{Context, IntoElement, Render, Window, div, prelude::*};

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

/// One row of the list, as a GPUI view.
pub struct RowView {
    pub row: ProcessRow,
}

impl Render for RowView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .gap_4()
            .child(div().debug_selector(|| "cell-name".into()).child(self.row.name.clone()))
            .child(div().debug_selector(|| "cell-cpu".into()).child(self.row.cpu.clone()))
            .child(div().debug_selector(|| "cell-memory".into()).child(self.row.memory.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Bounds, Pixels, TestAppContext, VisualTestContext};

    fn open(cx: &mut TestAppContext) -> &mut VisualTestContext {
        let chrome = Process { pid: 4120, name: "chrome.exe".to_string(), cpu: 12.46, memory_bytes: 536_870_912 };
        let (_view, cx) = cx.add_window_view(|_, _| RowView { row: ProcessRow::from_process(&chrome) });
        cx.run_until_parked();
        cx
    }

    fn cell(cx: &mut VisualTestContext, name: &'static str) -> Bounds<Pixels> {
        cx.debug_bounds(name).unwrap_or_else(|| panic!("no box named {name} was drawn"))
    }

    #[gpui::test]
    fn row_draws_all_three_cells(cx: &mut TestAppContext) {
        let cx = open(cx);
        for name in ["cell-name", "cell-cpu", "cell-memory"] {
            cell(cx, name);
        }
    }

    #[gpui::test]
    fn cells_sit_left_to_right(cx: &mut TestAppContext) {
        let cx = open(cx);
        let (name, cpu, memory) = (cell(cx, "cell-name"), cell(cx, "cell-cpu"), cell(cx, "cell-memory"));
        assert!(cpu.left() >= name.right(), "cell-cpu should sit right of cell-name, not under it");
        assert!(memory.left() >= cpu.right(), "cell-memory should sit right of cell-cpu");
        assert_eq!(cpu.top(), name.top(), "all three cells should be on one line");
        assert_eq!(memory.top(), name.top(), "all three cells should be on one line");
    }
}
