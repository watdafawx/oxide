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

pub use bar::{StatsBar, clamp_interval, wanted_kinds};
pub use thermal::missing_from_wmi;
pub use real::{CpuSensor, RamSensor};
pub use sensor::{ScriptedSensor, Sensor, Missing, missing_text};
pub use view::{LabelMode, Look};
pub use net::{NetSpeedSensor, counts_toward_total, speed};
pub use disk::DiskSensor;
pub use gpu::{GpuInfo, GpuSensor, NvidiaGpu, Vendor, pick_gpu};
pub use thermal::{ThermalSensor, acpi_to_celsius};
pub use power::{Battery, BatterySensor, decode_battery, read_power_status};
pub use ping::{PingSensor, ping_once};
pub use uptime::format_uptime;

use std::collections::HashSet;
use std::sync::Mutex;

use gpui::Context;

/// One read of one sensor: the stat, its value, and why there is none.
pub type SensorRead = (StatKind, Option<f64>, Option<Missing>);

/// Read every wanted sensor once, with the reason for each gap. Runs on a background thread.
pub fn read_sensors(sensors: &Mutex<Vec<Box<dyn Sensor>>>, wanted: &HashSet<StatKind>) -> Vec<SensorRead> {
    let mut sensors = sensors.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    sensors
        .iter_mut()
        .filter(|s| wanted.contains(&s.kind()))
        .map(|s| {
            let value = s.read();
            (s.kind(), value, if value.is_none() { s.missing() } else { None })
        })
        .collect()
}

impl StatsBar {
    /// Store one round of reads: values, histories, reasons; redraw if the bar changed.
    pub fn apply(&mut self, reads: Vec<SensorRead>, cx: &mut Context<Self>) {
        for (kind, value, missing) in reads {
            match value {
                Some(v) => {
                    self.snapshot.insert(kind, v);
                    self.histories.entry(kind).or_insert_with(|| History::new(60)).push(v);
                    self.missing.remove(&kind);
                }
                None => {
                    self.snapshot.remove(&kind);
                    match missing {
                        Some(why) => self.missing.insert(kind, why),
                        None => self.missing.remove(&kind),
                    };
                }
            }
        }
        self.ticks += 1;
        let items = bar_items(&self.order, &self.snapshot, &Limits::default());
        if items != self.items {
            self.items = items;
            cx.notify();
        }
    }

    /// The hover card's text for a stat: its summary, or why it has no number.
    pub fn hover_text(&self, kind: StatKind) -> String {
        if let Some(why) = self.missing.get(&kind) {
            return missing_text(*why).to_string();
        }
        match self.histories.get(&kind) {
            Some(history) => hover_summary(kind, history),
            None => "no data yet".to_string(),
        }
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

    /// A scripted sensor that also counts reads and knows why it has no number.
    struct Fake {
        kind: StatKind,
        values: Vec<Option<f64>>,
        why: Option<Missing>,
        reads: Arc<AtomicU32>,
    }

    impl Fake {
        fn new(kind: StatKind, values: Vec<Option<f64>>, why: Option<Missing>) -> (Self, Arc<AtomicU32>) {
            let reads = Arc::new(AtomicU32::new(0));
            (Fake { kind, values, why, reads: reads.clone() }, reads)
        }
    }

    impl Sensor for Fake {
        fn kind(&self) -> StatKind {
            self.kind
        }
        fn read(&mut self) -> Option<f64> {
            let n = self.reads.fetch_add(1, Ordering::SeqCst) as usize;
            self.values.get(n).copied().flatten()
        }
        fn missing(&self) -> Option<Missing> {
            self.why
        }
    }

    fn all_wanted() -> std::collections::HashSet<StatKind> {
        stat::ALL.into_iter().collect()
    }

    #[test]
    fn hub_reads_cpu_and_ram() {
        let sensors: Vec<Box<dyn Sensor>> = vec![Box::new(CpuSensor::new()), Box::new(RamSensor::new())];
        let reads = read_sensors(&std::sync::Mutex::new(sensors), &all_wanted());
        assert_eq!(reads.len(), 2);
        assert!(reads.iter().all(|(_, value, _)| value.is_some()), "CPU and RAM always read: {reads:?}");
    }

    #[test]
    fn hidden_stats_cost_nothing() {
        let (gpu, gpu_reads) = Fake::new(StatKind::GpuTemp, vec![Some(60.0)], None);
        let (cpu, cpu_reads) = Fake::new(StatKind::CpuUsage, vec![Some(5.0)], None);
        let sensors: Vec<Box<dyn Sensor>> = vec![Box::new(gpu), Box::new(cpu)];
        let wanted = wanted_kinds(&[Slot::Stat(StatKind::CpuUsage)]);
        let reads = read_sensors(&std::sync::Mutex::new(sensors), &wanted);
        assert_eq!(reads, vec![(StatKind::CpuUsage, Some(5.0), None)]);
        assert_eq!((gpu_reads.load(Ordering::SeqCst), cpu_reads.load(Ordering::SeqCst)), (0, 1));
    }

    #[test]
    fn gaps_come_with_a_reason() {
        let (bat, _) = Fake::new(StatKind::Battery, vec![None], Some(Missing::NoDevice));
        let (cpu, _) = Fake::new(StatKind::CpuUsage, vec![Some(9.0)], Some(Missing::Timeout));
        let sensors: Vec<Box<dyn Sensor>> = vec![Box::new(bat), Box::new(cpu)];
        let reads = read_sensors(&std::sync::Mutex::new(sensors), &all_wanted());
        assert_eq!(reads[0], (StatKind::Battery, None, Some(Missing::NoDevice)));
        assert_eq!(reads[1], (StatKind::CpuUsage, Some(9.0), None), "a value needs no reason");
    }

    #[gpui::test]
    fn bar_shows_hub_readings(cx: &mut TestAppContext) {
        let (cpu, _) = Fake::new(StatKind::CpuUsage, vec![Some(10.0), Some(30.0)], None);
        let bar = cx.new(|cx| {
            let mut bar = StatsBar::new(vec![Box::new(cpu)]);
            bar.start(cx);
            bar
        });
        wait(cx, 1000);
        wait(cx, 1000);
        assert_eq!(bar.read_with(cx, |bar, _| bar.text_of(StatKind::CpuUsage)), Some("30%".to_string()));
        assert_eq!(bar.read_with(cx, |bar, _| bar.hover_text(StatKind::CpuUsage)), "min 10% · avg 20% · max 30%");
    }

    #[gpui::test]
    fn bar_remembers_why(cx: &mut TestAppContext) {
        let (temp, _) = Fake::new(StatKind::CpuTemp, vec![None, Some(55.0)], Some(Missing::NeedsAdmin));
        let bar = cx.new(|cx| {
            let mut bar = StatsBar::new(vec![Box::new(temp)]);
            bar.start(cx);
            bar
        });
        wait(cx, 1000);
        assert_eq!(bar.read_with(cx, |bar, _| bar.text_of(StatKind::CpuTemp)), Some("--".to_string()));
        assert_eq!(bar.read_with(cx, |bar, _| bar.hover_text(StatKind::CpuTemp)), "needs administrator");
        wait(cx, 1000);
        assert_eq!(bar.read_with(cx, |bar, _| bar.hover_text(StatKind::CpuTemp)), "min 55°C · avg 55°C · max 55°C", "a value clears the reason");
    }

    #[gpui::test]
    fn unchanged_reads_skip_redraw(cx: &mut TestAppContext) {
        let (cpu, _) = Fake::new(StatKind::CpuUsage, vec![Some(4.0), Some(4.2), Some(3.9)], None);
        let bar = cx.new(|cx| {
            let mut bar = StatsBar::new(vec![Box::new(cpu)]);
            bar.start(cx);
            bar
        });
        let notifies = Rc::new(Cell::new(0));
        let seen = notifies.clone();
        cx.update(|cx| cx.observe(&bar, move |_, _| seen.set(seen.get() + 1)).detach());
        for _ in 0..3 {
            wait(cx, 1000);
        }
        assert_eq!(bar.read_with(cx, |bar, _| bar.ticks), 3);
        assert_eq!(notifies.get(), 1, "-- to 4% once; 4% stays 4%");
    }

    #[test]
    fn network_speed_needs_two_reads() {
        let mut down = NetSpeedSensor::download();
        assert_eq!(down.read(), None, "the first read only starts the clock");
        std::thread::sleep(std::time::Duration::from_millis(50));
        assert!(down.read().is_some_and(|v| v >= 0.0));
    }
}
