//! Kinetik Stats: screen time and alerts.

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
pub mod alerts;
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
pub mod screen;
pub mod screen_view;
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

#[allow(unused_imports)]
pub use alerts::*;
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
pub use screen::*;
#[allow(unused_imports)]
pub use screen_view::*;
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
#[allow(unused_imports)]
use chrono::{NaiveDate, NaiveDateTime, Timelike};
#[allow(unused_imports)]
use std::collections::{BTreeMap, HashMap, HashSet};
#[allow(unused_imports)]
use std::path::{Path, PathBuf};
#[allow(unused_imports)]
use std::time::{Duration, Instant};

impl ScreenTime {
    /// One sample: a new day starts a new count, and the program in front, if it counts, gets 5 seconds, saved straight away.
    pub fn sample(&mut self, desk: &dyn Desk) {
        let _ = desk;
        // TODO: a new date starts a new Day and prunes; a program to count is added and saved
    }
}

impl Render for ScreenTimeView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let _ = cx;
        // TODO: summarize today's Day; five summary lines, then 24 bars in a chart CHART_HEIGHT tall
        div()
    }
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

    /// 22 October 2026 at h:m:s.
    fn at(h: u32, m: u32, s: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2026, 10, 22).unwrap().and_hms_opt(h, m, s).unwrap()
    }

    /// A fresh, empty folder for one test's day files.
    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("kinetik-screen-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        dir
    }

    /// Chrome 08:00 to 08:30 and Minecraft 20:30 to 22:15, a sample every 5 seconds.
    fn evening_day() -> Day {
        let mut day = Day::new(at(0, 0, 0).date());
        for (app, from, until) in [("chrome.exe", at(8, 0, 0), at(8, 30, 0)), ("minecraft.windows.exe", at(20, 30, 0), at(22, 15, 0))] {
            let mut time = from;
            while time < until {
                day.add(app, time);
                time += chrono::TimeDelta::seconds(5);
            }
        }
        day
    }

    /// Opens the screen time page over `day`.
    fn screen_page(cx: &mut TestAppContext, day: Day) -> (Entity<ScreenTime>, VisualTestContext) {
        let screen = cx.new(|_| {
            let mut screen = ScreenTime::new(scratch("page"), day.date);
            screen.day = day;
            screen
        });
        let mut slot = None;
        cx.update(|cx| toggle_screen_window(cx, &mut slot, screen.clone()));
        let handle = slot.expect("the screen time page opened");
        let mut vcx = VisualTestContext::from_window(handle.into(), cx);
        vcx.run_until_parked();
        (screen, vcx)
    }

    fn bounds(cx: &mut VisualTestContext, name: &str) -> gpui::Bounds<gpui::Pixels> {
        let name: &'static str = Box::leak(name.to_string().into_boxed_str());
        cx.debug_bounds(name).unwrap_or_else(|| panic!("nothing named {name} was drawn"))
    }

    /// A bar whose GPU temperature reads `temps`, one a second, watched by alerts that only remember.
    fn hot_bar(cx: &mut TestAppContext, temps: Vec<f64>) -> (Entity<StatsBar>, Entity<Alerts>) {
        cx.update(|cx| cx.set_global(Settings::default()));
        let bar = cx.new(|cx| {
            let sensors: Vec<Box<dyn Sensor>> = vec![Box::new(ScriptedSensor::new(StatKind::GpuTemp, temps))];
            let mut bar = StatsBar::from_settings(sensors, cx);
            bar.start(cx);
            bar
        });
        let alerts = cx.update(|cx| watch_alerts(&bar, |_| {}, cx));
        (bar, alerts)
    }

    fn run_seconds(cx: &mut TestAppContext, seconds: u32) {
        for _ in 0..seconds {
            cx.executor().advance_clock(Duration::from_secs(1));
            cx.run_until_parked();
        }
    }

    fn fired(cx: &mut TestAppContext, alerts: &Entity<Alerts>) -> usize {
        alerts.read_with(cx, |alerts, _| alerts.fired.len())
    }

    #[gpui::test]
    fn counts_foreground_app(cx: &mut TestAppContext) {
        let desk = FakeDesk::new("C:\\Games\\Minecraft.Windows.exe", at(21, 0, 0));
        let screen = cx.new(|cx| {
            let mut screen = ScreenTime::new(scratch("counts"), at(21, 0, 0).date());
            screen.start(Box::new(desk), cx);
            screen
        });
        for _ in 0..3 {
            cx.executor().advance_clock(Duration::from_secs(SAMPLE_SECS));
            cx.run_until_parked();
        }
        screen.read_with(cx, |screen, _| assert_eq!(screen.day.apps.get("minecraft.windows.exe"), Some(&15)));
    }

    #[test]
    fn idle_and_lock_pause() {
        let mut screen = ScreenTime::new(scratch("pause"), at(10, 0, 0).date());
        let mut desk = FakeDesk::new("C:\\Apps\\Code.exe", at(10, 0, 0));
        screen.sample(&desk);
        desk.idle = Duration::from_secs(6 * 60);
        screen.sample(&desk);
        desk.fullscreen = true;
        screen.sample(&desk);
        desk.locked = true;
        screen.sample(&desk);
        assert_eq!(screen.day.apps.get("code.exe"), Some(&10), "counted while used and while full screen; not idle, not locked");
    }

    #[test]
    fn files_per_day_pruned() {
        let dir = scratch("days");
        std::fs::create_dir_all(&dir).unwrap();
        let old = at(0, 0, 0).date() - chrono::TimeDelta::days(401);
        std::fs::write(day_path(&dir, old), "{}").unwrap();
        let mut screen = ScreenTime::new(dir.clone(), at(23, 59, 55).date());
        let mut desk = FakeDesk::new("C:\\Windows\\notepad.exe", at(23, 59, 55));
        screen.sample(&desk);
        desk.now = at(23, 59, 55) + chrono::TimeDelta::seconds(5);
        screen.sample(&desk);
        let tomorrow = desk.now.date();
        assert!(day_path(&dir, at(0, 0, 0).date()).exists(), "today has its file");
        assert!(day_path(&dir, tomorrow).exists(), "after midnight a new file starts");
        assert_eq!(screen.day.date, tomorrow);
        assert!(!day_path(&dir, old).exists(), "a new day prunes the old files");
    }

    #[gpui::test]
    fn summary_right(cx: &mut TestAppContext) {
        let summary = summarize(&evening_day());
        assert_eq!((summary.total, summary.busiest_hour, summary.most_used.as_deref()), (8100, Some(21), Some("minecraft.windows.exe")));
        let (_, mut vcx) = screen_page(cx, evening_day());
        for name in ["total", "first", "last", "most-used", "busiest"] {
            bounds(&mut vcx, name);
        }
    }

    #[gpui::test]
    fn chart_drawn(cx: &mut TestAppContext) {
        let (_, mut vcx) = screen_page(cx, evening_day());
        let chart = bounds(&mut vcx, "chart");
        let busiest = bounds(&mut vcx, "hour-21");
        assert_eq!(busiest.size.height, px(CHART_HEIGHT));
        assert_eq!(busiest.bottom(), chart.bottom(), "bars stand on the bottom edge");
        assert!(bounds(&mut vcx, "hour-0").origin.x < bounds(&mut vcx, "hour-23").origin.x);
    }

    #[gpui::test]
    fn alerts_debounced(cx: &mut TestAppContext) {
        let (_bar, alerts) = hot_bar(cx, [vec![90.0; 20], vec![60.0; 5], vec![90.0; 60]].concat());
        run_seconds(cx, 24);
        assert_eq!(fired(cx, &alerts), 0);
        run_seconds(cx, 60);
        assert_eq!(fired(cx, &alerts), 1);
    }
}
