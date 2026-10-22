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

pub use bar::{StatsBar, clamp_interval, wanted_kinds};
pub use real::{CpuSensor, RamSensor};
pub use sensor::{ScriptedSensor, Sensor};
pub use view::{LabelMode, Look};
pub use net::{NetSpeedSensor, counts_toward_total, speed};
pub use disk::DiskSensor;
pub use gpu::{GpuInfo, GpuSensor, NvidiaGpu, Vendor, pick_gpu};
pub use thermal::ThermalSensor;

/// ACPI reports tenths of a kelvin. These are values desktops report when they have no real sensor.
const DUMMY: [u32; 3] = [2732, 2982, 3010];

/// Tenths of a kelvin to °C, or None for a known dummy value or anything outside 1 to 120 °C.
pub fn acpi_to_celsius(tenths_kelvin: u32) -> Option<f64> {
    let _ = DUMMY;
    Some(tenths_kelvin as f64) // TODO: None for DUMMY values; tenths / 10 - 273.15; None outside 1..=120 °C
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
    fn kelvin_tenths_to_celsius() {
        let c = acpi_to_celsius(3332).expect("3332 is a real temperature");
        assert!((c - 60.05).abs() < 1e-9, "got {c}");
    }

    #[test]
    fn dummy_27_8_is_ignored() {
        for dummy in [3010, 2982, 2732] {
            assert_eq!(acpi_to_celsius(dummy), None, "{dummy} is a dummy value");
        }
        assert_eq!(acpi_to_celsius(0), None);
        assert_eq!(acpi_to_celsius(5000), None, "226 °C is not a CPU");
    }
}
