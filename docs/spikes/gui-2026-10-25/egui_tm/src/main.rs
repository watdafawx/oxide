#![windows_subsystem = "windows"]
use eframe::egui;
use std::time::{Duration, Instant};
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System};

struct Row { pid: u32, name: String, cpu: f32, mem: u64 }

struct App { sys: System, rows: Vec<Row>, last: Instant }

impl App {
    fn refresh(&mut self) {
        self.sys.refresh_processes_specifics(ProcessesToUpdate::All, true, ProcessRefreshKind::nothing().with_cpu().with_memory());
        self.rows = self.sys.processes().values().map(|p| Row {
            pid: p.pid().as_u32(), name: p.name().to_string_lossy().into_owned(), cpu: p.cpu_usage(), mem: p.memory(),
        }).collect();
        self.rows.sort_by(|a, b| b.cpu.total_cmp(&a.cpu));
        self.last = Instant::now();
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if self.last.elapsed() >= Duration::from_secs(1) { self.refresh(); }
        ui.ctx().request_repaint_after(Duration::from_secs(1));
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.label(format!("{} processes", self.rows.len()));
            egui_extras::TableBuilder::new(ui)
                .column(egui_extras::Column::remainder())
                .columns(egui_extras::Column::auto(), 3)
                .header(20.0, |mut h| { for t in ["Name", "PID", "CPU", "Memory"] { h.col(|ui| { ui.strong(t); }); } })
                .body(|body| body.rows(18.0, self.rows.len(), |mut row| {
                    let r = &self.rows[row.index()];
                    row.col(|ui| { ui.label(&r.name); });
                    row.col(|ui| { ui.label(r.pid.to_string()); });
                    row.col(|ui| { ui.label(format!("{:.1}%", r.cpu)); });
                    row.col(|ui| { ui.label(format!("{} MB", r.mem / 1_048_576)); });
                }));
        });
    }
}

fn main() -> eframe::Result {
    let mut app = App { sys: System::new(), rows: vec![], last: Instant::now() };
    app.refresh();
    eframe::run_native("egui_tm", eframe::NativeOptions::default(), Box::new(|_| Ok(Box::new(app))))
}
