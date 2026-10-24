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

/// Bar heights for the 24 hours: the busiest hour gets `tallest`, the others their share of it. An empty day is all zeros.
pub fn bar_heights(hours: &[u64; 24], tallest: f32) -> [f32; 24] {
    let _ = (hours, tallest);
    [0.0; 24] // TODO: the busiest hour gets tallest, the others their share of it; an empty day stays flat

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

    #[gpui::test]
    fn busiest_bar_is_tallest(cx: &mut TestAppContext) {
        let heights = bar_heights(&evening_day().hours, 100.0);
        assert_eq!((heights[21], heights[8], heights[22], heights[3]), (100.0, 50.0, 25.0, 0.0));
        assert_eq!(bar_heights(&[0; 24], 100.0), [0.0; 24]);
        let (_, mut vcx) = screen_page(cx, evening_day());
        let tallest = bounds(&mut vcx, "hour-21").size.height;
        assert_eq!(tallest, px(CHART_HEIGHT));
        for hour in [8, 20, 22] {
            assert!(bounds(&mut vcx, &format!("hour-{hour}")).size.height < tallest, "hour {hour} is shorter");
        }
    }
}
