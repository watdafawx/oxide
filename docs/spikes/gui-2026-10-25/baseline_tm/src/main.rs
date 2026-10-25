#![windows_subsystem = "windows"]
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System};
fn main() {
    let mut sys = System::new();
    loop {
        sys.refresh_processes_specifics(ProcessesToUpdate::All, true, ProcessRefreshKind::nothing().with_cpu().with_memory());
        let mut rows: Vec<(String, f32)> = sys.processes().values().map(|p| (p.name().to_string_lossy().into_owned(), p.cpu_usage())).collect();
        rows.sort_by(|a, b| b.1.total_cmp(&a.1));
        std::hint::black_box(&rows);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
