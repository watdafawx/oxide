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
pub mod net;

pub use bar::{StatsBar, clamp_interval, wanted_kinds};
pub use real::{CpuSensor, RamSensor};
pub use sensor::{ScriptedSensor, Sensor};
pub use view::{LabelMode, Look};
pub use net::NetSpeedSensor;

use std::time::Duration;

/// Bytes counted since the last read, over the time since the last read.
pub fn speed(bytes: u64, elapsed: Duration) -> f64 {
    let seconds = elapsed.as_secs_f64();
    if seconds == 0.0 { 0.0 } else { bytes as f64 / seconds }
}

/// Every sensor this PC has so far; the rest of the bar is scripted.
pub fn live_sensors() -> Vec<Box<dyn Sensor>> {
    let mut sensors: Vec<Box<dyn Sensor>> = vec![
        Box::new(CpuSensor::new()),
        Box::new(RamSensor::new()),
        Box::new(NetSpeedSensor::download()),
        Box::new(NetSpeedSensor::upload()),
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
    fn speed_is_bytes_over_seconds() {
        assert_eq!(speed(3_000_000, Duration::from_secs(2)), 1_500_000.0);
        assert_eq!(speed(500, Duration::from_millis(500)), 1000.0);
        assert_eq!(speed(500, Duration::ZERO), 0.0);
    }
}
