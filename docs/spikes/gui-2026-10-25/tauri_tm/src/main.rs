#![windows_subsystem = "windows"]
use std::sync::Mutex;
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System};

#[derive(serde::Serialize)]
struct Row { pid: u32, name: String, cpu: f32, mem: u64 }

#[tauri::command]
fn processes(sys: tauri::State<Mutex<System>>) -> Vec<Row> {
    let mut sys = sys.lock().unwrap();
    sys.refresh_processes_specifics(ProcessesToUpdate::All, true, ProcessRefreshKind::nothing().with_cpu().with_memory());
    let mut rows: Vec<Row> = sys.processes().values().map(|p| Row {
        pid: p.pid().as_u32(), name: p.name().to_string_lossy().into_owned(), cpu: p.cpu_usage(), mem: p.memory(),
    }).collect();
    rows.sort_by(|a, b| b.cpu.total_cmp(&a.cpu));
    rows
}

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(System::new()))
        .invoke_handler(tauri::generate_handler![processes])
        .run(tauri::generate_context!())
        .unwrap();
}
