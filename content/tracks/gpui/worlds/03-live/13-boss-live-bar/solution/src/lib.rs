//! Kinetik Stats: the model, the bar, and now it ticks.

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
pub mod view;

pub use view::{LabelMode, Look};
use std::collections::HashMap;
use std::time::Duration;

use gpui::{Context, Task};

pub mod real;
pub mod sensor;

pub use sensor::{ScriptedSensor, Sensor};

/// Scripted sensors for every stat on the default bar: a few minutes of a busy PC.
pub fn demo_sensors() -> Vec<Box<dyn Sensor>> {
    let wave = |base: f64, step: f64| (0..300).map(|i| base + step * ((i % 7) as f64)).collect::<Vec<_>>();
    vec![
        Box::new(ScriptedSensor::new(StatKind::CpuUsage, wave(8.0, 4.0))),
        Box::new(ScriptedSensor::new(StatKind::CpuTemp, wave(70.0, 3.0))),
        Box::new(ScriptedSensor::new(StatKind::GpuUsage, wave(60.0, 6.0))),
        Box::new(ScriptedSensor::new(StatKind::GpuTemp, wave(64.0, 1.0))),
        Box::new(ScriptedSensor::new(StatKind::RamUsed, wave(8.0e9, 1.0e8))),
        Box::new(ScriptedSensor::new(StatKind::NetDown, wave(1.0e6, 6.0e5))),
        Box::new(ScriptedSensor::new(StatKind::NetUp, wave(2.0e4, 1.5e4))),
        Box::new(ScriptedSensor::new(StatKind::Ping, wave(18.0, 3.0))),
    ]
}

/// Kinetik's update interval setting, in milliseconds, kept between 0.5 and 5 seconds.
pub fn clamp_interval(ms: u64) -> Duration {
    Duration::from_millis(ms.clamp(500, 5000))
}

pub use real::{CpuSensor, RamSensor};

/// The bar of stats, reading its sensors on a timer.
pub struct StatsBar {
    pub snapshot: Snapshot,
    pub items: Vec<BarItem>,
    pub look: Look,
    pub sensors: Vec<Box<dyn Sensor>>,
    pub interval: Duration,
    /// How many times the sensors were read.
    pub ticks: u32,
    pub histories: HashMap<StatKind, History>,
    _poll: Option<Task<()>>,
}

impl StatsBar {
    /// A bar that reads `sensors` once a second. Every stat shows -- until the first tick.
    pub fn new(sensors: Vec<Box<dyn Sensor>>) -> Self {
        let snapshot = Snapshot::new();
        StatsBar {
            items: bar_items(&default_order(), &snapshot, &Limits::default()),
            snapshot,
            look: Look::default(),
            sensors,
            interval: Duration::from_secs(1),
            ticks: 0,
            histories: HashMap::new(),
            _poll: None,
        }
    }

    /// The text the pill for `kind` shows, if it is on the bar.
    pub fn text_of(&self, kind: StatKind) -> Option<String> {
        self.items.iter().find_map(|item| match item {
            BarItem::Pill(pill) if pill.kind == kind => Some(pill.text.clone()),
            _ => None,
        })
    }

    /// The same bar with the interval setting (in milliseconds) applied, clamped.
    pub fn with_interval(mut self, ms: u64) -> Self {
        self.interval = clamp_interval(ms);
        self
    }

    /// Read every sensor once: snapshot, histories, ticks; redraw only if the items changed.
    pub fn tick(&mut self, cx: &mut Context<Self>) {
        for sensor in &mut self.sensors {
            let kind = sensor.kind();
            match sensor.read() {
                Some(v) => {
                    self.snapshot.insert(kind, v);
                    self.histories.entry(kind).or_insert_with(|| History::new(60)).push(v);
                }
                None => {
                    self.snapshot.remove(&kind);
                }
            }
        }
        self.ticks += 1;
        let items = bar_items(&default_order(), &self.snapshot, &Limits::default());
        if items != self.items {
            self.items = items;
            cx.notify();
        }
    }

    /// Read the sensors every interval until the bar is gone.
    pub fn start(&mut self, cx: &mut Context<Self>) {
        let interval = self.interval;
        let task = cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor().timer(interval).await;
                if this.update(cx, |bar, cx| bar.tick(cx)).is_err() {
                    break;
                }
            }
        });
        self._poll = Some(task);
    }
}

/// Real CPU and RAM, scripted everything else.
pub fn live_sensors() -> Vec<Box<dyn Sensor>> {
    let mut sensors = demo_sensors();
    sensors.retain(|s| s.kind() != StatKind::CpuUsage && s.kind() != StatKind::RamUsed);
    sensors.insert(0, Box::new(CpuSensor::new()));
    sensors.insert(1, Box::new(RamSensor::new()));
    sensors
}

#[cfg(test)]
#[allow(unused_imports, dead_code)]
mod tests {
    use super::*;
    use gpui::{TestAppContext, prelude::*};
    use std::{cell::Cell, rc::Rc};

    /// Counts how often `bar` calls cx.notify().
    fn count_notifies(cx: &mut TestAppContext, bar: &gpui::Entity<StatsBar>) -> Rc<Cell<u32>> {
        let n = Rc::new(Cell::new(0));
        let seen = n.clone();
        cx.update(|cx| cx.observe(bar, move |_, _| seen.set(seen.get() + 1)).detach());
        n
    }

    fn wait(cx: &mut TestAppContext, ms: u64) {
        cx.executor().advance_clock(std::time::Duration::from_millis(ms));
        cx.run_until_parked();
    }

    fn cpu_script(values: &[f64]) -> Vec<Box<dyn Sensor>> {
        vec![Box::new(ScriptedSensor::new(StatKind::CpuUsage, values.to_vec()))]
    }

    fn started(cx: &mut TestAppContext, sensors: Vec<Box<dyn Sensor>>, ms: u64) -> gpui::Entity<StatsBar> {
        cx.new(|cx| {
            let mut bar = StatsBar::new(sensors).with_interval(ms);
            bar.start(cx);
            bar
        })
    }

    /// A sensor that counts its reads, so a test can see when reading stops.
    struct Counting(Rc<Cell<u32>>);

    impl Sensor for Counting {
        fn kind(&self) -> StatKind {
            StatKind::Ping
        }
        fn read(&mut self) -> Option<f64> {
            self.0.set(self.0.get() + 1);
            Some(20.0)
        }
    }

    #[gpui::test]
    fn bar_starts_with_dashes(cx: &mut TestAppContext) {
        let bar = started(cx, cpu_script(&[42.0]), 1000);
        cx.run_until_parked();
        assert_eq!(bar.read_with(cx, |bar, _| bar.text_of(StatKind::CpuUsage)), Some("--".to_string()));
    }

    #[gpui::test]
    fn first_tick_fills_cpu(cx: &mut TestAppContext) {
        let bar = started(cx, cpu_script(&[42.0]), 1000);
        wait(cx, 1000);
        assert_eq!(bar.read_with(cx, |bar, _| bar.text_of(StatKind::CpuUsage)), Some("42%".to_string()));
    }

    #[gpui::test]
    fn ticks_follow_interval(cx: &mut TestAppContext) {
        let bar = started(cx, cpu_script(&[1.0, 2.0, 3.0]), 2000);
        wait(cx, 1999);
        assert_eq!(bar.read_with(cx, |bar, _| bar.ticks), 0, "a 2 s interval has not passed yet");
        wait(cx, 1);
        wait(cx, 2000);
        assert_eq!(bar.read_with(cx, |bar, _| bar.ticks), 2);
        let tiny = started(cx, cpu_script(&[1.0]), 10);
        wait(cx, 499);
        assert_eq!(tiny.read_with(cx, |bar, _| bar.ticks), 0, "10 ms is clamped to 500 ms");
    }

    #[gpui::test]
    fn history_grows_per_tick(cx: &mut TestAppContext) {
        let bar = started(cx, cpu_script(&[10.0, 20.0, 30.0]), 1000);
        for _ in 0..3 {
            wait(cx, 1000);
        }
        let summary = bar.read_with(cx, |bar, _| hover_summary(StatKind::CpuUsage, &bar.histories[&StatKind::CpuUsage]));
        assert_eq!(summary, "min 10% · avg 20% · max 30%");
    }

    #[gpui::test]
    fn unchanged_tick_skips_redraw(cx: &mut TestAppContext) {
        let bar = started(cx, cpu_script(&[7.0, 7.0, 7.0, 8.0]), 1000);
        let notifies = count_notifies(cx, &bar);
        for _ in 0..4 {
            wait(cx, 1000);
        }
        assert_eq!(notifies.get(), 2, "-- to 7% and 7% to 8% are the only changes");
    }

    #[gpui::test]
    fn closing_stops_the_loop(cx: &mut TestAppContext) {
        let reads = Rc::new(Cell::new(0));
        let bar = started(cx, vec![Box::new(Counting(reads.clone()))], 1000);
        wait(cx, 2000);
        assert_eq!(reads.get(), 2);
        drop(bar);
        cx.run_until_parked();
        wait(cx, 3000);
        assert_eq!(reads.get(), 2, "the bar is gone, so nothing should read any more");
    }
}
