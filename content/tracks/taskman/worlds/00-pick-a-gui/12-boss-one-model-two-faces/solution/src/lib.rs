//! The task manager's list: one model, drawn by egui and by GPUI.

pub mod egui_face;
pub mod gpui_face;
pub mod model;

pub use model::{Process, ProcessRow, SortBy, TaskList};

/// Five processes to look at, for the tests and for `cargo run`.
pub fn sample_processes() -> Vec<Process> {
    vec![
        Process { pid: 4, name: "explorer.exe".to_string(), cpu: 0.5, memory_bytes: 180 << 20 },
        Process { pid: 4120, name: "chrome.exe".to_string(), cpu: 12.5, memory_bytes: 512 << 20 },
        Process { pid: 9020, name: "game.exe".to_string(), cpu: 40.0, memory_bytes: 3 << 30 },
        Process { pid: 4188, name: "Chrome Helper.exe".to_string(), cpu: 2.0, memory_bytes: 96 << 20 },
        Process { pid: 7310, name: "Zoom.exe".to_string(), cpu: 1.0, memory_bytes: 250 << 20 },
    ]
}
