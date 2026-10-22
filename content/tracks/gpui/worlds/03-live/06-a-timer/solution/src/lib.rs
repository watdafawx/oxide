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
use std::time::Duration;

use gpui::{Context, Task};

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

/// The bar of stats, reading its sensors on a timer.
pub struct StatsBar {
    pub snapshot: Snapshot,
    pub items: Vec<BarItem>,
    pub look: Look,
    pub sensors: Vec<Box<dyn Sensor>>,
    pub interval: Duration,
    /// How many times the sensors were read.
    pub ticks: u32,
    _poll: Option<Task<()>>,
}

impl StatsBar {
    /// A bar that reads `sensors`. Every stat shows -- until the first tick.
    pub fn new(sensors: Vec<Box<dyn Sensor>>) -> Self {
        let snapshot = Snapshot::new();
        StatsBar {
            items: bar_items(&default_order(), &snapshot, &Limits::default()),
            snapshot,
            look: Look::default(),
            sensors,
            interval: Duration::from_secs(1),
            ticks: 0,
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

    /// Read every sensor once and rebuild the items.
    pub fn tick(&mut self, cx: &mut Context<Self>) {
        for sensor in &mut self.sensors {
            match sensor.read() {
                Some(v) => self.snapshot.insert(sensor.kind(), v),
                None => self.snapshot.remove(&sensor.kind()),
            };
        }
        self.items = bar_items(&default_order(), &self.snapshot, &Limits::default());
        self.ticks += 1;
        cx.notify();
    }

    /// Start reading the sensors.
    pub fn start(&mut self, cx: &mut Context<Self>) {
        let interval = self.interval;
        let task = cx.spawn(async move |this, cx| {
            cx.background_executor().timer(interval).await;
            this.update(cx, |bar, cx| bar.tick(cx)).ok();
        });
        self._poll = Some(task);
    }
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

    #[gpui::test]
    fn first_read_waits_one_interval(cx: &mut TestAppContext) {
        let sensors: Vec<Box<dyn Sensor>> = vec![Box::new(ScriptedSensor::new(StatKind::CpuUsage, vec![42.0]))];
        let bar = cx.new(|cx| {
            let mut bar = StatsBar::new(sensors);
            bar.start(cx);
            bar
        });
        wait(cx, 999);
        assert_eq!(bar.read_with(cx, |bar, _| bar.ticks), 0, "read too early");
        wait(cx, 1);
        assert_eq!(bar.read_with(cx, |bar, _| bar.ticks), 1, "should have read at 1 s");
    }
}
