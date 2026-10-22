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

/// Programs that fill the screen without being games.
pub const NOT_GAMES: [&str; 3] = ["explorer.exe", "LockApp.exe", "ShellExperienceHost.exe"];

/// A game, for the overlay: the window in front covers its monitor (fullscreen or borderless),
/// and it is not Windows' own desktop or lock screen.
pub fn is_game(window: Rect, monitor: Rect, exe: &str) -> bool {
    covers_monitor(window, monitor) && !NOT_GAMES.iter().any(|name| exe.eq_ignore_ascii_case(name))
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

    fn monitor() -> Rect {
        Rect::new(0, 0, 2560, 1440)
    }

    #[test]
    fn borderless_game_counts() {
        assert!(is_game(monitor(), monitor(), "Cyberpunk2077.exe"));
    }

    #[test]
    fn windowed_app_does_not() {
        assert!(!is_game(Rect::new(100, 100, 1380, 820), monitor(), "Minecraft.exe"));
        assert!(!is_game(monitor(), monitor(), "Explorer.EXE"));
    }
}
