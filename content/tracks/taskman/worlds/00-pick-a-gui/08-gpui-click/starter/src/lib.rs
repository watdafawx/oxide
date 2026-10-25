//! The task manager's process list in GPUI, with a CPU header that sorts it.

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

/// The whole list, as a GPUI view.
pub struct TaskListView {
    pub processes: Vec<Process>,
    /// How many times `render` ran. The tests read it.
    pub renders: usize,
}

impl TaskListView {
    /// Highest CPU first.
    pub fn sort_by_cpu(&mut self) {
        self.processes.sort_by(|a, b| b.cpu.total_cmp(&a.cpu));
    }

    /// This second's numbers arrive (later worlds call this from a timer).
    pub fn set_processes(&mut self, processes: Vec<Process>, cx: &mut Context<Self>) {
        self.processes = processes;
        // TODO: tell GPUI the view changed, so the new numbers get drawn.
    }
}

/// One row: name, CPU and memory, side by side.
fn row(row: ProcessRow) -> impl IntoElement {
    div().flex().gap_4().child(row.name).child(row.cpu).child(row.memory)
}

impl Render for TaskListView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.renders += 1;
        let header = div()
            .id("header-cpu")
            .debug_selector(|| "header-cpu".into())
            .child("CPU");
        // TODO: when the header is clicked, sort the list by CPU,
        // then tell GPUI the view changed.
        div()
            .flex()
            .flex_col()
            .child(header)
            .children(self.processes.iter().map(|p| row(ProcessRow::from_process(p))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Entity, Modifiers, TestAppContext, VisualTestContext};

    fn process(name: &str, cpu: f32) -> Process {
        Process { pid: 1, name: name.to_string(), cpu, memory_bytes: 100 << 20 }
    }

    fn open(cx: &mut TestAppContext) -> (Entity<TaskListView>, &mut VisualTestContext) {
        let processes = vec![process("explorer.exe", 0.5), process("chrome.exe", 12.5), process("game.exe", 40.0)];
        let (list, cx) = cx.add_window_view(|_, _| TaskListView { processes, renders: 0 });
        cx.run_until_parked();
        (list, cx)
    }

    fn click_cpu(cx: &mut VisualTestContext) {
        let at = cx.debug_bounds("header-cpu").expect("no box named header-cpu was drawn").center();
        cx.simulate_click(at, Modifiers::none());
        cx.run_until_parked();
    }

    #[gpui::test]
    fn clicking_cpu_puts_busiest_first(cx: &mut TestAppContext) {
        let (list, cx) = open(cx);
        click_cpu(cx);
        let names: Vec<String> = list.read_with(cx, |list, _| list.processes.iter().map(|p| p.name.clone()).collect());
        assert_eq!(names, ["game.exe", "chrome.exe", "explorer.exe"]);
    }

    #[gpui::test]
    fn new_numbers_redraw_the_list(cx: &mut TestAppContext) {
        let (list, cx) = open(cx);
        let before = list.read_with(cx, |list, _| list.renders);
        list.update(cx, |list, cx| list.set_processes(vec![process("idle.exe", 0.0)], cx));
        cx.run_until_parked();
        let after = list.read_with(cx, |list, _| list.renders);
        assert!(after > before, "render did not run again after set_processes: does it call cx.notify()?");
    }
}
