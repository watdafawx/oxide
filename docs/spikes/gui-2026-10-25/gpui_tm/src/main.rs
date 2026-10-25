#![windows_subsystem = "windows"]
use std::{ops::Range, time::Duration};
use gpui::{App, Application, Bounds, Context, SharedString, Window, WindowBounds, WindowOptions, div, prelude::*, px, rgb, size, uniform_list};
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System};

struct Row { pid: u32, name: SharedString, cpu: f32, mem: u64 }

struct TaskList { sys: System, rows: Vec<Row> }

impl TaskList {
    fn refresh(&mut self) {
        self.sys.refresh_processes_specifics(ProcessesToUpdate::All, true, ProcessRefreshKind::nothing().with_cpu().with_memory());
        self.rows = self.sys.processes().values().map(|p| Row {
            pid: p.pid().as_u32(), name: p.name().to_string_lossy().into_owned().into(), cpu: p.cpu_usage(), mem: p.memory(),
        }).collect();
        self.rows.sort_by(|a, b| b.cpu.total_cmp(&a.cpu));
    }
}

impl Render for TaskList {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().bg(rgb(0x1e1e1e)).text_color(rgb(0xdddddd)).text_sm().flex().flex_col()
            .child(format!("{} processes", self.rows.len()))
            .child(uniform_list("procs", self.rows.len(), cx.processor(|this, range: Range<usize>, _, _| {
                range.map(|i| { let r = &this.rows[i];
                    div().flex().h(px(18.)).child(div().w(px(260.)).child(r.name.clone()))
                        .child(div().w(px(70.)).child(r.pid.to_string()))
                        .child(div().w(px(70.)).child(format!("{:.1}%", r.cpu)))
                        .child(div().w(px(90.)).child(format!("{} MB", r.mem / 1_048_576)))
                }).collect()
            })).flex_grow())
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(640.), px(480.)), cx);
        cx.open_window(WindowOptions { window_bounds: Some(WindowBounds::Windowed(bounds)), ..Default::default() }, |_, cx| {
            cx.new(|cx| {
                let mut list = TaskList { sys: System::new(), rows: vec![] };
                list.refresh();
                cx.spawn(async move |this, cx| loop {
                    cx.background_executor().timer(Duration::from_secs(1)).await;
                    if this.update(cx, |l: &mut TaskList, cx| { l.refresh(); cx.notify(); }).is_err() { break; }
                }).detach();
                list
            })
        }).unwrap();
        cx.on_window_closed(|cx| cx.quit()).detach();
    });
}
