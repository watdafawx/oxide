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

impl Render for DashboardView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let settings = cx.global::<Settings>().dashboard.clone();
        let width = window.viewport_size().width;
        let columns = columns_for(width);
        let card_width = (width - px(32.)) / columns as f32;
        let t = theme(settings.theme);
        let bar = self.bar.read(cx);
        let cards: Vec<Card> = visible_cards(&settings.hidden)
            .into_iter()
            .map(|kind| {
                let value = bar.snapshot.get(&kind).copied();
                Card {
                    kind,
                    numbers: card_numbers(bar, kind),
                    share: value.and_then(|v| bar_share(kind, v)),
                    history: bar.histories.get(&kind).map(|h| h.values().collect()).unwrap_or_default(),
                    theme: t,
                    text_size: card_text_size(card_width),
                }
            })
            .collect();
        div()
            .debug_selector(|| "dashboard".into())
            .size_full()
            .p_4()
            .bg(rgb(t.background))
            .grid()
            .grid_cols(columns as u16)
            .gap_4()
            .children(cards)
    }
}

/// Run the web dashboard on its own thread with its own tokio runtime: GPUI's executor is not tokio.
pub fn serve_dashboard(addr: SocketAddr, state: WebState) -> std::thread::JoinHandle<std::io::Result<()>> {
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new()?;
        runtime.block_on(async move {
            let listener = tokio::net::TcpListener::bind(addr).await?;
            axum::serve(listener, router(state)).await
        })
    })
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

    #[gpui::test]
    fn window_cards_fit(cx: &mut TestAppContext) {
        let bar = ticking_bar(cx);
        let (_, mut vcx) = dashboard(cx, bar);
        vcx.simulate_resize(size(px(1000.), px(800.)));
        vcx.run_until_parked();
        let row = bounds(&mut vcx, "card-CpuUsage").origin.y;
        assert_eq!(bounds(&mut vcx, "card-RamUsed").origin.y, row, "three columns at 1000 px");
        assert!(bounds(&mut vcx, "card-GpuUsage").origin.y > row);
    }

    #[gpui::test]
    fn cards_show_now_avg_peak(cx: &mut TestAppContext) {
        let bar = ticking_bar(cx);
        let (_, mut vcx) = dashboard(cx, bar);
        for name in ["now-CpuUsage", "avg-CpuUsage", "peak-CpuUsage", "gauge-CpuUsage"] {
            assert!(vcx.debug_bounds(name).is_some(), "{name} missing");
        }
    }

    #[gpui::test]
    fn layout_editor_applies(cx: &mut TestAppContext) {
        let bar = ticking_bar(cx);
        cx.update(|cx| Settings::update_global(cx, |s, _| s.dashboard.hidden = vec![StatKind::Ping, StatKind::Battery]));
        let (_, mut vcx) = dashboard(cx, bar);
        assert!(vcx.debug_bounds("card-Ping").is_none() && vcx.debug_bounds("card-Battery").is_none());
        assert!(vcx.debug_bounds("card-NetDown").is_some());
    }

    #[gpui::test]
    fn theme_switches(cx: &mut TestAppContext) {
        let bar = ticking_bar(cx);
        cx.update(|cx| Settings::update_global(cx, |s, _| s.dashboard.theme = DashTheme::Amber));
        let (_, mut vcx) = dashboard(cx, bar);
        assert!(vcx.debug_bounds("dashboard").is_some());
        assert!(vcx.debug_bounds("card-CpuUsage").is_some(), "amber still draws every card");
    }

    #[gpui::test]
    fn web_json_matches_window(cx: &mut TestAppContext) {
        let bar = ticking_bar(cx);
        let (web, card) = bar.read_with(cx, |bar, _| (web_snapshot(bar), card_numbers(bar, StatKind::GpuTemp)));
        let gpu = web.stats.iter().find(|s| s.kind == StatKind::GpuTemp).unwrap();
        assert_eq!(gpu.text, card.now, "the web page and the window agree");
    }

    #[test]
    fn key_required() {
        assert_eq!(ask(state_with("k"), "/api/stats").0, StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn server_answers_on_localhost() {
        use std::io::{Read, Write};
        let port = std::net::TcpListener::bind("127.0.0.1:0").unwrap().local_addr().unwrap().port();
        let addr = bind_addr(false, port);
        let _server = serve_dashboard(addr, state_with("k"));
        let mut reply = String::new();
        for _ in 0..50 {
            if let Ok(mut stream) = std::net::TcpStream::connect(addr) {
                stream.write_all(b"GET /api/stats?key=k HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n").unwrap();
                stream.read_to_string(&mut reply).unwrap();
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
        assert!(reply.starts_with("HTTP/1.1 200"), "got: {reply}");
        assert!(reply.contains("CpuUsage"));
    }
}
