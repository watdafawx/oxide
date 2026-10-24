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
pub mod net;

pub use bar::{StatsBar, clamp_interval, wanted_kinds};
pub use real::{CpuSensor, RamSensor};
pub use sensor::{ScriptedSensor, Sensor};
pub use view::{LabelMode, Look};
pub use net::{NetSpeedSensor, counts_toward_total, speed};
pub use disk::DiskSensor;

use nvml_wrapper::Nvml;
use nvml_wrapper::enum_wrappers::device::TemperatureSensor;

/// An NVIDIA card through the driver's nvml.dll, or nothing when there is none.
pub struct NvidiaGpu {
    nvml: Option<Nvml>,
}

impl NvidiaGpu {
    /// Loads nvml.dll. On a PC without an NVIDIA driver this is a GPU with no readings.
    pub fn new() -> Self {
        NvidiaGpu { nvml: Nvml::init().ok() }
    }

    /// A GPU with no NVIDIA card behind it, for tests.
    pub fn none() -> Self {
        NvidiaGpu { nvml: None }
    }

    pub fn is_present(&self) -> bool {
        self.nvml.is_some()
    }

    /// The first card's usage, 0 to 100.
    pub fn usage(&self) -> Option<f64> {
        todo!("the first card, as in temperature; then its utilization rates; None at the first gap")
    }

    /// The first card's temperature in °C.
    pub fn temperature(&self) -> Option<f64> {
        let device = self.nvml.as_ref()?.device_by_index(0).ok()?;
        Some(device.temperature(TemperatureSensor::Gpu).ok()? as f64)
    }
}

impl Default for NvidiaGpu {
    fn default() -> Self {
        NvidiaGpu::new()
    }
}

/// GPU usage or GPU temperature from an NvidiaGpu.
pub struct GpuSensor {
    gpu: NvidiaGpu,
    kind: StatKind,
}

impl GpuSensor {
    pub fn usage() -> Self {
        GpuSensor { gpu: NvidiaGpu::new(), kind: StatKind::GpuUsage }
    }

    pub fn temperature() -> Self {
        GpuSensor { gpu: NvidiaGpu::new(), kind: StatKind::GpuTemp }
    }
}

impl Sensor for GpuSensor {
    fn kind(&self) -> StatKind {
        self.kind
    }

    fn read(&mut self) -> Option<f64> {
        if self.kind == StatKind::GpuUsage { self.gpu.usage() } else { self.gpu.temperature() }
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
    fn no_nvidia_means_none() {
        let gpu = NvidiaGpu::none();
        assert_eq!(gpu.usage(), None);
        assert_eq!(gpu.temperature(), None);
    }

    #[test]
    fn nvidia_usage_is_a_percentage() {
        let gpu = NvidiaGpu::new();
        if !gpu.is_present() {
            println!("skip: no NVIDIA driver on this PC");
            return;
        }
        let usage = gpu.usage().expect("an NVIDIA card should report usage");
        assert!((0.0..=100.0).contains(&usage), "usage {usage}");
    }
}
