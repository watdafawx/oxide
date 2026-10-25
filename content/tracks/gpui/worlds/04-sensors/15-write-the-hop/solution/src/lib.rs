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
pub mod uptime;

pub use bar::{StatsBar, clamp_interval, wanted_kinds, SensorRead, read_sensors};
pub use real::{CpuSensor, RamSensor};
pub use sensor::{ScriptedSensor, Sensor};
pub use view::{LabelMode, Look};
pub use net::{NetSpeedSensor, counts_toward_total, speed};
pub use disk::DiskSensor;
pub use gpu::{GpuInfo, GpuSensor, NvidiaGpu, Vendor, pick_gpu};
pub use thermal::{ThermalSensor, acpi_to_celsius};
pub use power::{Battery, BatterySensor, decode_battery, read_power_status};
pub use ping::{PingSensor, ping_once};
pub use uptime::format_uptime;

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

    /// Counts its reads from any thread.
    struct Counting(StatKind, Arc<AtomicU32>);

    impl Sensor for Counting {
        fn kind(&self) -> StatKind {
            self.0
        }
        fn read(&mut self) -> Option<f64> {
            self.1.fetch_add(1, Ordering::SeqCst);
            Some(42.0)
        }
    }

    #[gpui::test]
    fn bar_shows_reads_from_the_background(cx: &mut TestAppContext) {
        let reads = Arc::new(AtomicU32::new(0));
        let sensors: Vec<Box<dyn Sensor>> = vec![Box::new(Counting(StatKind::CpuUsage, reads.clone()))];
        let bar = cx.new(|cx| {
            let mut bar = StatsBar::new(sensors);
            bar.start(cx);
            bar
        });
        for _ in 0..2 {
            wait(cx, 1000);
        }
        assert_eq!(reads.load(Ordering::SeqCst), 2);
        assert_eq!(bar.read_with(cx, |bar, _| bar.text_of(StatKind::CpuUsage)), Some("42%".to_string()));
        assert_eq!(bar.read_with(cx, |bar, _| bar.ticks), 2);
    }
}
