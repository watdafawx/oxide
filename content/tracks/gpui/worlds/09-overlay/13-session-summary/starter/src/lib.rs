//! Kinetik Stats: the game overlay.

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
pub mod widget;
pub mod win;
pub mod window;

pub use bar::{SensorRead, StatsBar, clamp_interval, read_sensors, wanted_kinds};
pub use disk::DiskSensor;
pub use dock::{Call, FakeWin, Layer, Rect, Taskbar, WinApi, bar_visible, covers_monitor, dock_bar, taskbars_to_use, to_logical};
pub use docker::Docker;
pub use frames::*;
pub use gpu::{GpuSensor, NvidiaGpu};
pub use interact::*;
pub use live::*;
pub use net::NetSpeedSensor;
#[allow(unused_imports)]
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
pub use widget::*;
pub use win::{RealWin, find_taskbar, hwnd_of};
pub use window::bar_window_options;

#[allow(unused_imports)]
use std::collections::VecDeque;
#[allow(unused_imports)]
use global_hotkey::{GlobalHotKeyEvent, HotKeyState, hotkey::{Code, HotKey, Modifiers}};
#[allow(unused_imports)]
use gpui::{Bounds, Pixels, point, px, size};
#[allow(unused_imports)]
use ferrisetw::{native::EvntraceNativeError, trace::TraceError};

/// The first line of Sessions.csv.
pub const SESSIONS_HEADER: &str = "game,minutes,average_fps,one_percent_low,peak_gpu_temp";

/// What a finished game session looked like.
#[derive(Debug, Clone, PartialEq)]
pub struct SessionSummary {
    pub game: String,
    pub seconds: u64,
    pub average_fps: f64,
    pub one_percent_low: f64,
    pub peak_gpu_temp: Option<f64>,
}

impl SessionSummary {
    /// One CSV line, in the header's order: minutes and FPS with one decimal, a missing temperature empty.
    pub fn csv_line(&self) -> String {
        todo!("game, minutes, average FPS, 1% low with one decimal; the temperature, or empty")
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
    use gpui::{TestAppContext, VisualTestContext, prelude::*};

    /// The capture in fixtures/: 1200 frames of a game with frame generation, five stutters.
    fn capture() -> Vec<Frame> {
        parse_capture(include_str!("../fixtures/capture.csv"))
    }

    fn capture_ms() -> Vec<f64> {
        capture().iter().map(|f| f.ms).collect()
    }

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 0.01
    }

    fn summary(temp: Option<f64>) -> SessionSummary {
        SessionSummary { game: "Cyberpunk2077.exe".into(), seconds: 2550, average_fps: 118.06, one_percent_low: 64.31, peak_gpu_temp: temp }
    }

    #[test]
    fn summary_line_matches_header() {
        let line = summary(Some(71.4)).csv_line();
        assert_eq!(line, "Cyberpunk2077.exe,42.5,118.1,64.3,71");
        assert_eq!(line.split(',').count(), SESSIONS_HEADER.split(',').count());
        assert_eq!(summary(None).csv_line(), "Cyberpunk2077.exe,42.5,118.1,64.3,");
    }
}
