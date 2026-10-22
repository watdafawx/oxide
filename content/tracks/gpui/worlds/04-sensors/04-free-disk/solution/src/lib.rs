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
pub use net::{NetSpeedSensor, counts_toward_total, speed};

use std::path::Path;

use sysinfo::Disks;

/// Free space on C:, in bytes.
pub struct DiskSensor {
    disks: Disks,
}

impl DiskSensor {
    pub fn new() -> Self {
        DiskSensor { disks: Disks::new_with_refreshed_list() }
    }
}

impl Default for DiskSensor {
    fn default() -> Self {
        DiskSensor::new()
    }
}

impl Sensor for DiskSensor {
    fn kind(&self) -> StatKind {
        StatKind::DiskFree
    }

    fn read(&mut self) -> Option<f64> {
        self.disks.refresh(true);
        let c = self.disks.list().iter().find(|d| d.mount_point() == Path::new("C:\\"))?;
        Some(c.available_space() as f64)
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
    fn c_drive_has_free_space() {
        let free = DiskSensor::new().read().expect("this PC has a C: drive");
        let disks = Disks::new_with_refreshed_list();
        let total = disks.list().iter().find(|d| d.mount_point() == Path::new("C:\\")).unwrap().total_space();
        assert!(free > 0.0 && free <= total as f64, "free {free} should be within 0..={total}");
    }
}
