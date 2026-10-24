//! The live bar: sensors read in the background every interval.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use gpui::{Context, Task};

use crate::sensor::Sensor;
use crate::{BarItem, History, Limits, Look, Slot, Snapshot, StatKind, bar_items, default_order};

/// Kinetik's update interval setting, in milliseconds, kept between 0.5 and 5 seconds.
pub fn clamp_interval(ms: u64) -> Duration {
    Duration::from_millis(ms.clamp(500, 5000))
}

/// The stats the bar shows: only these are worth reading.
pub fn wanted_kinds(order: &[Slot]) -> HashSet<StatKind> {
    order
        .iter()
        .filter_map(|slot| match slot {
            Slot::Stat(kind) => Some(*kind),
            Slot::Divider(_) => None,
        })
        .collect()
}

/// One read of one sensor: the stat and its value.
pub type SensorRead = (StatKind, Option<f64>);

/// Read every wanted sensor once. Runs on a background thread, so it may take its time.
pub fn read_sensors(sensors: &Mutex<Vec<Box<dyn Sensor>>>, wanted: &HashSet<StatKind>) -> Vec<SensorRead> {
    let mut sensors = sensors.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    sensors
        .iter_mut()
        .filter(|s| wanted.contains(&s.kind()))
        .map(|s| (s.kind(), s.read()))
        .collect()
}

/// The bar of stats. Its sensors are read on a background thread every interval.
pub struct StatsBar {
    pub order: Vec<Slot>,
    pub snapshot: Snapshot,
    pub items: Vec<BarItem>,
    pub look: Look,
    pub sensors: Arc<Mutex<Vec<Box<dyn Sensor>>>>,
    pub interval: Duration,
    /// How many times the sensors were read.
    pub ticks: u32,
    pub histories: HashMap<StatKind, History>,
    _poll: Option<Task<()>>,
}

impl StatsBar {
    /// A bar that reads `sensors` once a second. Every stat shows -- until the first read.
    pub fn new(sensors: Vec<Box<dyn Sensor>>) -> Self {
        let order = default_order();
        let snapshot = Snapshot::new();
        StatsBar {
            items: bar_items(&order, &snapshot, &Limits::default()),
            order,
            snapshot,
            look: Look::default(),
            sensors: Arc::new(Mutex::new(sensors)),
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

    /// The same bar showing `order` instead of the default.
    pub fn with_order(mut self, order: Vec<Slot>) -> Self {
        self.order = order;
        self.items = bar_items(&self.order, &self.snapshot, &Limits::default());
        self
    }

    /// Every interval: read the wanted sensors in the background, then apply on the UI thread.
    pub fn start(&mut self, cx: &mut Context<Self>) {
        let interval = self.interval;
        let task = cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor().timer(interval).await;
                let Ok((sensors, wanted)) = this.read_with(cx, |bar, _| (bar.sensors.clone(), wanted_kinds(&bar.order)))
                else {
                    break;
                };
                // TODO: run read_sensors(&sensors, &wanted) on the background executor and wait for its reads.
                // TODO: then, back on the UI thread, hand the reads to `apply`; stop the loop when the bar is gone.
                let _ = (sensors, wanted);
            }
        });
        self._poll = Some(task);
    }

    /// Store one round of reads: values and histories; redraw if the bar changed.
    pub fn apply(&mut self, reads: Vec<SensorRead>, cx: &mut Context<Self>) {
        for (kind, value) in reads {
            match value {
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
        let items = bar_items(&self.order, &self.snapshot, &Limits::default());
        if items != self.items {
            self.items = items;
            cx.notify();
        }
    }
}
