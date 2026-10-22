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

pub use bar::{StatsBar, clamp_interval, wanted_kinds};
pub use real::{CpuSensor, RamSensor};
pub use sensor::{ScriptedSensor, Sensor};
pub use view::{LabelMode, Look};
pub use net::{NetSpeedSensor, counts_toward_total, speed};
pub use disk::DiskSensor;
pub use gpu::{GpuSensor, NvidiaGpu};

/// Who made a graphics card.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vendor {
    Nvidia,
    Amd,
    Intel,
}

/// One graphics card in this PC.
#[derive(Debug, Clone, PartialEq)]
pub struct GpuInfo {
    pub name: String,
    pub vendor: Vendor,
    pub vram_bytes: u64,
}

/// Kinetik's "Auto" choice: the NVIDIA card if there is one, else the most video memory.
pub fn pick_gpu(gpus: &[GpuInfo]) -> Option<usize> {
    let _ = gpus;
    None // TODO: the first NVIDIA card's index (position), else the index with the most vram_bytes
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

    fn card(name: &str, vendor: Vendor, gb: u64) -> GpuInfo {
        GpuInfo { name: name.into(), vendor, vram_bytes: gb << 30 }
    }

    #[test]
    fn auto_prefers_nvidia() {
        let gpus = [card("Intel UHD 770", Vendor::Intel, 1), card("RTX 5070 Ti", Vendor::Nvidia, 16)];
        assert_eq!(pick_gpu(&gpus), Some(1));
        let gpus = [card("RTX 3060", Vendor::Nvidia, 12), card("RX 7900", Vendor::Amd, 24)];
        assert_eq!(pick_gpu(&gpus), Some(0), "NVIDIA wins even with less VRAM");
    }

    #[test]
    fn else_most_vram_wins() {
        let gpus = [card("Intel Arc A380", Vendor::Intel, 6), card("RX 7800", Vendor::Amd, 16)];
        assert_eq!(pick_gpu(&gpus), Some(1));
        assert_eq!(pick_gpu(&[]), None);
    }
}
