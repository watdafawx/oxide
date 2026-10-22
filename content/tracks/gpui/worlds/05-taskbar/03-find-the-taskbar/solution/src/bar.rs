//! The live bar: sensors read in the background every interval.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use gpui::{Context, Task};

use crate::sensor::{Missing, Sensor, missing_text};
use crate::{BarItem, History, Limits, Look, Slot, Snapshot, StatKind, bar_items, default_order, hover_summary};

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

/// One read of one sensor: the stat, its value, and why there is none.
pub type SensorRead = (StatKind, Option<f64>, Option<Missing>);

/// Read every wanted sensor once. Runs on a background thread, so it may take its time.
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
    /// Why each missing stat is missing, for the hover card.
    pub missing: HashMap<StatKind, Missing>,
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
            missing: HashMap::new(),
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
                let reads = cx.background_executor().spawn(async move { read_sensors(&sensors, &wanted) }).await;
                if this.update(cx, |bar, cx| bar.apply(reads, cx)).is_err() {
                    break;
                }
            }
        });
        self._poll = Some(task);
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
}
