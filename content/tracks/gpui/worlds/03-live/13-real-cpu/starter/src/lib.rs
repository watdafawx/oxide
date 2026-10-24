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

    /// Read every sensor once and rebuild the items.
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

    /// Start reading the sensors.
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

use sysinfo::System;

/// CPU usage from Windows, through sysinfo. The first read after creating it is 0:
/// usage is measured between two refreshes.
pub struct CpuSensor {
    system: System,
}

impl CpuSensor {
    pub fn new() -> Self {
        CpuSensor { system: System::new() }
    }
}

impl Default for CpuSensor {
    fn default() -> Self {
        CpuSensor::new()
    }
}

impl Sensor for CpuSensor {
    fn kind(&self) -> StatKind {
        StatKind::CpuUsage
    }

    fn read(&mut self) -> Option<f64> {
        None // TODO: take a fresh CPU measurement, then give back the usage across all cores, as f64
    }
}

/// Real CPU, scripted everything else.
pub fn live_sensors() -> Vec<Box<dyn Sensor>> {
    let mut sensors = demo_sensors();
    sensors.retain(|s| s.kind() != StatKind::CpuUsage);
    sensors.insert(0, Box::new(CpuSensor::new()));
    sensors
}

#[cfg(test)]
#[allow(unused_imports, dead_code)]
mod tests {
    use super::*;
    use gpui::{TestAppContext, prelude::*};
    use std::{cell::Cell, rc::Rc};

    #[test]
    fn cpu_usage_is_a_percentage() {
        let mut cpu = CpuSensor::new();
        cpu.read();
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        let usage = cpu.read().expect("CpuSensor should give a value");
        assert!((0.0..=100.0).contains(&usage), "CPU usage {usage} is not a percentage");
    }
}
