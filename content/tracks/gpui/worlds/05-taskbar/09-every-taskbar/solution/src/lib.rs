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
pub mod docker;
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
pub use dock::{Call, FakeWin, Rect, TRAY_GAP, Taskbar, WinApi, bar_visible, covers_monitor, dock_bar, to_logical};
pub use docker::Docker;
pub use win::{RealWin, find_taskbar, hwnd_of};
pub use window::bar_window_options;

/// The taskbars that get a bar: the primary one, or all of them with "Show on all taskbars".
pub fn taskbars_to_use(taskbars: &[Taskbar], all_taskbars: bool) -> Vec<Taskbar> {
    taskbars.iter().filter(|t| all_taskbars || t.primary).copied().collect()
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

    fn two() -> Vec<Taskbar> {
        let second = Taskbar {
            rect: Rect::new(2560, 1032, 4480, 1080),
            tray: Rect::new(4480, 1032, 4480, 1080),
            monitor: Rect::new(2560, 0, 4480, 1080),
            scale: 1.0,
            primary: false,
        };
        vec![taskbar(), second]
    }

    #[test]
    fn two_taskbars_two_bars() {
        assert_eq!(taskbars_to_use(&two(), true).len(), 2);
    }

    #[test]
    fn primary_only_by_default() {
        assert_eq!(taskbars_to_use(&two(), false), vec![taskbar()]);
    }
}
