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
pub use dock::{Call, FakeWin, Rect, TRAY_GAP, Taskbar, WinApi, bar_visible, covers_monitor, dock_bar, taskbars_to_use, to_logical};
pub use docker::Docker;
pub use win::{RealWin, find_taskbar, hwnd_of};
pub use window::bar_window_options;

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
    fn moved_taskbar_redocks_bar() {
        let mut d = Docker::new(FakeWin { taskbars: vec![taskbar()], ..Default::default() }, 7, 300);
        assert_eq!(d.redock(), Some(Rect::new(1732, 1392, 2032, 1440)));
        assert_eq!(d.redock(), None, "nothing changed");
        let wider_tray = Taskbar { tray: Rect::new(1940, 1392, 2560, 1440), ..taskbar() };
        d.api.taskbars = vec![wider_tray];
        assert_eq!(d.redock(), Some(Rect::new(1632, 1392, 1932, 1440)));
        assert_eq!(d.api.calls, vec![Call::Move(7, 1732, 1392), Call::Move(7, 1632, 1392)]);
    }
}
