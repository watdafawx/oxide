//! Kinetik Stats: the bar, and the sensors that feed it.

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
pub mod real;
pub mod sensor;
pub mod view;
pub mod disk;
pub mod gpu;
pub mod net;
pub mod thermal;
pub mod power;
pub mod ping;

pub use bar::{StatsBar, clamp_interval, wanted_kinds};
pub use real::{CpuSensor, RamSensor};
pub use sensor::{ScriptedSensor, Sensor};
pub use view::{LabelMode, Look};
pub use net::{NetSpeedSensor, counts_toward_total, speed};
pub use disk::DiskSensor;
pub use gpu::{GpuInfo, GpuSensor, NvidiaGpu, Vendor, pick_gpu};
pub use thermal::{ThermalSensor, acpi_to_celsius};
pub use power::{Battery, BatterySensor, decode_battery, read_power_status};
pub use ping::{PingSensor, ping_once};

/// How long the PC has been on: "3d 4h", "4h 12m" under a day, "12m" under an hour.
pub fn format_uptime(seconds: u64) -> String {
    format!("{seconds}s") // TODO: days, hours, minutes; then "3d 4h", "4h 12m" or "12m"
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
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::{cell::Cell, rc::Rc};

    fn wait(cx: &mut TestAppContext, ms: u64) {
        cx.executor().advance_clock(std::time::Duration::from_millis(ms));
        cx.run_until_parked();
    }

    #[test]
    fn uptime_shows_days_and_hours() {
        assert_eq!(format_uptime(277_200), "3d 5h");
        assert_eq!(format_uptime(15_120), "4h 12m");
        assert_eq!(format_uptime(720), "12m");
        assert_eq!(format_uptime(30), "0m");
    }
}
