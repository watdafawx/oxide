//! Kinetik Stats: the dashboard, in a window and on the web.

pub mod demo;
pub mod format;
pub mod history;
pub mod limits;
pub mod order;
pub mod pills;
pub mod reading;
pub mod stat;

pub use format::{format_bytes, format_percent, format_speed};
pub use history::{History, Summary};
pub use limits::{Limit, Limits, WarnWhen, is_warning};
pub use order::{DividerStyle, Slot, default_order, move_up};
pub use pills::{BarItem, Column, Pill, Snapshot, bar_items, hover_summary, two_line_columns};
pub use reading::Reading;
pub use stat::{StatKind, Unit};
pub mod bar;
pub mod dashboard;
pub mod disk;
pub mod dock;
pub mod docker;
pub mod frames;
pub mod gpu;
pub mod interact;
pub mod live;
pub mod net;
pub mod overlay;
pub mod ping;
pub mod power;
pub mod real;
pub mod sensor;
pub mod settings;
pub mod settings_window;
pub mod thermal;
pub mod ui;
pub mod uptime;
pub mod view;
pub mod web;
pub mod widget;
pub mod win;
pub mod window;

pub use bar::{SensorRead, StatsBar, clamp_interval, read_sensors, wanted_kinds};
pub use dashboard::*;
pub use disk::DiskSensor;
pub use dock::{Call, FakeWin, Layer, Rect, Taskbar, WinApi, bar_visible, covers_monitor, dock_bar, taskbars_to_use, to_logical};
pub use docker::Docker;
pub use frames::*;
pub use gpu::{GpuSensor, NvidiaGpu};
pub use interact::*;
pub use live::*;
pub use net::NetSpeedSensor;
pub use overlay::*;
pub use ping::PingSensor;
pub use power::BatterySensor;
pub use real::{CpuSensor, RamSensor};
pub use sensor::{Missing, ScriptedSensor, Sensor, missing_text};
pub use settings::*;
pub use settings_window::*;
pub use thermal::ThermalSensor;
pub use ui::{BarUi, FakeLauncher, LeftClick, Launcher, ProcessUse, RealLauncher};
pub use view::{LabelMode, Look};
#[allow(unused_imports)]
pub use web::*;
pub use widget::*;
pub use win::{RealWin, find_taskbar, hwnd_of};
pub use window::bar_window_options;

#[allow(unused_imports)]
use std::net::{Ipv4Addr, SocketAddr};
#[allow(unused_imports)]
use std::sync::{Arc, Mutex};
#[allow(unused_imports)]
use axum::{Json, Router, extract::{Query, State}, http::StatusCode, response::{Html, IntoResponse, Response}, routing::get};
#[allow(unused_imports)]
use gpui::{App, Context, Div, Entity, IntoElement, PathBuilder, Pixels, Point, Render, RenderOnce, Window, canvas, div, point, prelude::*, px, rgb, size};
#[allow(unused_imports)]
use serde::Serialize;
#[allow(unused_imports)]
use crate::stat::ALL;

/// Where the web dashboard listens: only this PC (127.0.0.1), or every network card (0.0.0.0) when shared.
pub fn bind_addr(share_on_lan: bool, port: u16) -> SocketAddr {
    let _ = share_on_lan;
    SocketAddr::from((Ipv4Addr::UNSPECIFIED, port)) // TODO: 127.0.0.1 unless share_on_lan

}

/// Every sensor this PC has so far; the rest of the bar is scripted.
pub fn live_sensors() -> Vec<Box<dyn Sensor>> {
    let mut sensors: Vec<Box<dyn Sensor>> = vec![
        Box::new(CpuSensor::new()),
        Box::new(RamSensor::new()),
        Box::new(NetSpeedSensor::download()),
        Box::new(NetSpeedSensor::upload()),
        Box::new(DiskSensor::new()),
        Box::new(GpuSensor::usage()),
        Box::new(GpuSensor::temperature()),
        Box::new(ThermalSensor::new()),
        Box::new(BatterySensor),
        Box::new(PingSensor::new()),
    ];
    let real: Vec<StatKind> = sensors.iter().map(|s| s.kind()).collect();
    for kind in [StatKind::CpuUsage, StatKind::CpuTemp, StatKind::GpuUsage, StatKind::GpuTemp, StatKind::RamUsed, StatKind::NetDown, StatKind::NetUp, StatKind::Ping] {
        if !real.contains(&kind) {
            let script = (0..300).map(|i| 20.0 + (i % 7) as f64 * 5.0).collect();
            sensors.push(Box::new(ScriptedSensor::new(kind, script)));
        }
    }
    sensors
}

#[cfg(test)]
#[allow(unused_imports, dead_code, unused_variables, unused_mut)]
mod tests {
    use super::*;
    use gpui::{TestAppContext, UpdateGlobal, VisualTestContext, WindowHandle, prelude::*};
    use std::time::Duration;

    /// A bar that has read CPU 10, 20 and 30 %, GPU temperature 60 to 70 °C, and RAM.
    fn ticking_bar(cx: &mut TestAppContext) -> Entity<StatsBar> {
        cx.update(|cx| cx.set_global(Settings::default()));
        let bar = cx.new(|cx| {
            let sensors: Vec<Box<dyn Sensor>> = vec![
                Box::new(ScriptedSensor::new(StatKind::CpuUsage, vec![10.0, 20.0, 30.0])),
                Box::new(ScriptedSensor::new(StatKind::GpuTemp, vec![60.0, 70.0, 65.0])),
                Box::new(ScriptedSensor::new(StatKind::RamUsed, vec![8.0e9, 8.0e9, 8.0e9])),
            ];
            let mut bar = StatsBar::from_settings(sensors, cx);
            bar.start(cx);
            bar
        });
        for _ in 0..3 {
            cx.executor().advance_clock(Duration::from_secs(1));
            cx.run_until_parked();
        }
        bar
    }

    /// Opens the dashboard for `bar`.
    fn dashboard(cx: &mut TestAppContext, bar: Entity<StatsBar>) -> (WindowHandle<DashboardView>, VisualTestContext) {
        let mut slot = None;
        cx.update(|cx| toggle_dashboard_window(cx, &mut slot, bar));
        let handle = slot.expect("the dashboard opened");
        let mut vcx = VisualTestContext::from_window(handle.into(), cx);
        vcx.run_until_parked();
        (handle, vcx)
    }

    fn bounds(cx: &mut VisualTestContext, name: &'static str) -> gpui::Bounds<gpui::Pixels> {
        cx.debug_bounds(name).unwrap_or_else(|| panic!("nothing named {name} was drawn"))
    }

    /// Ask the router once, without a network: status and body text.
    fn ask(state: WebState, uri: &str) -> (StatusCode, String) {
        use tower::ServiceExt;
        let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
        runtime.block_on(async move {
            let request = axum::http::Request::get(uri).body(axum::body::Body::empty()).unwrap();
            let reply = router(state).oneshot(request).await.unwrap();
            let status = reply.status();
            let body = axum::body::to_bytes(reply.into_body(), usize::MAX).await.unwrap();
            (status, String::from_utf8(body.to_vec()).unwrap())
        })
    }

    fn state_with(key: &str) -> WebState {
        let snapshot = WebSnapshot { stats: vec![WebStat { kind: StatKind::CpuUsage, label: "CPU", text: "30%".into(), value: Some(30.0), warning: false }] };
        WebState { snapshot: Arc::new(Mutex::new(snapshot)), key: key.to_string() }
    }

    #[test]
    fn default_binds_localhost() {
        let settings = Settings::default();
        assert_eq!(bind_addr(settings.dashboard.share_on_lan, settings.dashboard.port), "127.0.0.1:8787".parse().unwrap());
        assert_eq!(bind_addr(true, 9000), "0.0.0.0:9000".parse().unwrap());
    }
}
