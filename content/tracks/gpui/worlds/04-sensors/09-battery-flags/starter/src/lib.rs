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
pub use thermal::{ThermalSensor, acpi_to_celsius};

use windows::Win32::System::Power::{GetSystemPowerStatus, SYSTEM_POWER_STATUS};

/// The laptop battery, decoded.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Battery {
    pub percent: f64,
    pub charging: bool,
}

/// Ask Windows for the power status. `unsafe`: Rust cannot check what the Windows function does.
pub fn read_power_status() -> Option<SYSTEM_POWER_STATUS> {
    let mut status = SYSTEM_POWER_STATUS::default();
    // SAFETY: `status` is a valid, writable SYSTEM_POWER_STATUS for the whole call.
    unsafe { GetSystemPowerStatus(&mut status) }.ok()?;
    Some(status)
}

/// The BatteryFlag bit worth 128 is "no system battery", the bit worth 8 is "charging"; a percent of 255 is "unknown".
pub fn decode_battery(status: &SYSTEM_POWER_STATUS) -> Option<Battery> {
    let _ = status;
    todo!("None when the bit worth 128 is set or the percent is 255; else the percent, and whether the bit worth 8 is set")
}

/// The laptop battery's charge.
#[derive(Default)]
pub struct BatterySensor;

impl Sensor for BatterySensor {
    fn kind(&self) -> StatKind {
        StatKind::Battery
    }

    fn read(&mut self) -> Option<f64> {
        read_power_status().as_ref().and_then(decode_battery).map(|b| b.percent)
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

    fn status(flag: u8, percent: u8) -> SYSTEM_POWER_STATUS {
        SYSTEM_POWER_STATUS { BatteryFlag: flag, BatteryLifePercent: percent, ..Default::default() }
    }

    #[test]
    fn this_pc_power_status_reads() {
        assert!(read_power_status().is_some(), "Windows should answer GetSystemPowerStatus");
    }

    #[test]
    fn no_battery_flag_means_none() {
        assert_eq!(decode_battery(&status(128, 255)), None, "a desktop");
        assert_eq!(decode_battery(&status(0, 255)), None, "unknown percent");
        assert_eq!(decode_battery(&status(8 | 2, 15)), Some(Battery { percent: 15.0, charging: true }));
        assert_eq!(decode_battery(&status(1, 80)), Some(Battery { percent: 80.0, charging: false }));
    }
}
