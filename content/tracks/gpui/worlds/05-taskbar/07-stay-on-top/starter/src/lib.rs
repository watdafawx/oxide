//! Kinetik Stats: the bar on the taskbar.

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
pub mod gpu;
pub mod net;
pub mod ping;
pub mod power;
pub mod real;
pub mod sensor;
pub mod thermal;
pub mod uptime;
pub mod view;
pub mod dock;
pub mod win;
pub mod window;

pub use bar::{SensorRead, StatsBar, clamp_interval, read_sensors, wanted_kinds};
pub use disk::DiskSensor;
pub use gpu::{GpuSensor, NvidiaGpu};
pub use net::NetSpeedSensor;
pub use ping::PingSensor;
pub use power::BatterySensor;
pub use real::{CpuSensor, RamSensor};
pub use sensor::{Missing, ScriptedSensor, Sensor, missing_text};
pub use thermal::ThermalSensor;
pub use view::{LabelMode, Look};
pub use dock::{Call, FakeWin, WinApi, Rect, TRAY_GAP, Taskbar, dock_bar, to_logical};
pub use win::{RealWin, find_taskbar, hwnd_of};
pub use window::bar_window_options;

/// Keeps one bar window on the taskbar.
pub struct Docker<A: WinApi> {
    pub api: A,
    pub hwnd: isize,
    pub bar_width: i32,
    pub shift_left: i32,
    /// The taskbar the bar was last placed against.
    pub last: Option<Taskbar>,
    pub visible: bool,
}

impl<A: WinApi> Docker<A> {
    pub fn new(api: A, hwnd: isize, bar_width: i32) -> Self {
        Docker { api, hwnd, bar_width, shift_left: 0, last: None, visible: true }
    }

    /// The bar window just opened: put it above the taskbar.
    pub fn on_open(&mut self) {
        // TODO: ask the api to make self.hwnd topmost
    }

    /// The window in front changed. Clicking the taskbar raises it over the bar, so raise the bar
    /// again whenever the taskbar comes to the front.
    pub fn on_foreground_change(&mut self, taskbar_in_front: bool) {
        let _ = taskbar_in_front; // TODO: when the taskbar is in front, make self.hwnd topmost again
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
#[allow(unused_imports, dead_code)]
mod tests {
    use super::*;
    use gpui::{TestAppContext, prelude::*};

    /// A 2560 x 1440 monitor with a 48 px taskbar at the bottom and a 520 px tray.
    fn taskbar() -> Taskbar {
        Taskbar {
            rect: Rect::new(0, 1392, 2560, 1440),
            tray: Rect::new(2040, 1392, 2560, 1440),
            monitor: Rect::new(0, 0, 2560, 1440),
            scale: 1.0,
            primary: true,
        }
    }

    #[test]
    fn topmost_set_after_open() {
        let mut d = Docker::new(FakeWin::default(), 42, 600);
        d.on_open();
        assert_eq!(d.api.calls, vec![Call::Topmost(42)]);
    }

    #[test]
    fn topmost_set_again_after_taskbar_click() {
        let mut d = Docker::new(FakeWin::default(), 42, 600);
        d.on_foreground_change(false);
        assert!(d.api.calls.is_empty(), "another window in front: nothing to do");
        d.on_foreground_change(true);
        assert_eq!(d.api.calls, vec![Call::Topmost(42)]);
    }
}
