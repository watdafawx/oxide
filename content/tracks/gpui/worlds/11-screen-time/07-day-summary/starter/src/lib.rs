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

/// Total, first and last use, the most used program and the busiest hour of a day.
pub fn summarize(day: &Day) -> DaySummary {
    DaySummary {
        total: day.apps.values().sum(),
        first: day.first,
        last: day.last,
        most_used: None,    // TODO: the name of the program with the most seconds
        busiest_hour: None, // TODO: the hour with the most seconds, skipping empty hours
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

    #[test]
    fn busiest_hour_is_21() {
        let summary = summarize(&evening_day());
        assert_eq!(summary.busiest_hour, Some(21));
        assert_eq!(hours_minutes(summary.total), "2 h 15 min");
        assert_eq!((summary.first.map(clock), summary.last.map(clock)), (Some("08:00".into()), Some("22:14".into())));
    }

    #[test]
    fn most_used_is_minecraft() {
        assert_eq!(summarize(&evening_day()).most_used.as_deref(), Some("minecraft.windows.exe"));
        let empty = summarize(&Day::new(at(0, 0, 0).date()));
        assert_eq!((empty.most_used, empty.busiest_hour), (None, None));
    }
}
