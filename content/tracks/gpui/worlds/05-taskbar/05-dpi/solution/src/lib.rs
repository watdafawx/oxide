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
pub use dock::{Rect, TRAY_GAP, Taskbar, dock_bar};
pub use win::find_taskbar;
pub use window::bar_window_options;

use gpui::{Bounds, Pixels, point, px, size};

/// Windows thinks in physical pixels; GPUI windows are placed in logical ones.
/// At 150 % scaling, 300 physical pixels are 200 logical.
pub fn to_logical(rect: Rect, scale: f32) -> Bounds<Pixels> {
    Bounds::new(
        point(px(rect.left as f32 / scale), px(rect.top as f32 / scale)),
        size(px(rect.width() as f32 / scale), px(rect.height() as f32 / scale)),
    )
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
mod tests {
    use super::*;

    #[test]
    fn bounds_scale_at_150_percent() {
        let b = to_logical(Rect::new(1500, 1392, 1800, 1440), 1.5);
        assert_eq!(b.origin, point(px(1000.), px(928.)));
        assert_eq!(b.size, size(px(200.), px(32.)));
        assert_eq!(to_logical(Rect::new(10, 20, 110, 68), 1.0).size, size(px(100.), px(48.)));
    }
}
