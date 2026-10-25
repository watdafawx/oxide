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
use gpui::Context;

/// The bar of stats, holding its own state.
pub struct StatsBar {
    pub snapshot: Snapshot,
    pub items: Vec<BarItem>,
    pub look: Look,
}

impl StatsBar {
    /// A bar showing `snapshot` in the default order.
    pub fn new(snapshot: Snapshot) -> Self {
        let items = bar_items(&default_order(), &snapshot, &Limits::default());
        StatsBar { snapshot, items, look: Look::default() }
    }

    /// The text the pill for `kind` shows, if it is on the bar.
    pub fn text_of(&self, kind: StatKind) -> Option<String> {
        self.items.iter().find_map(|item| match item {
            BarItem::Pill(pill) if pill.kind == kind => Some(pill.text.clone()),
            _ => None,
        })
    }

    /// A new value for one stat: store it, rebuild the items, ask for a redraw.
    pub fn set_reading(&mut self, kind: StatKind, value: f64, cx: &mut Context<Self>) {
        self.snapshot.insert(kind, value);
        self.items = bar_items(&default_order(), &self.snapshot, &Limits::default());
        cx.notify();
    }
}

/// Something that can read one stat. Real ones ask Windows; test ones play a script.
pub trait Sensor {
    /// Which stat this sensor reads.
    fn kind(&self) -> StatKind;
    /// The current value, or None when it cannot be read.
    fn read(&mut self) -> Option<f64>;
}

/// A stand-in sensor for tests and demos: gives its script's values in order, then None.
pub struct ScriptedSensor {
    pub kind: StatKind,
    pub script: Vec<f64>,
    pub next: usize,
}

impl ScriptedSensor {
    pub fn new(kind: StatKind, script: Vec<f64>) -> Self {
        ScriptedSensor { kind, script, next: 0 }
    }
}

impl Sensor for ScriptedSensor {
    fn kind(&self) -> StatKind {
        self.kind
    }

    fn read(&mut self) -> Option<f64> {
        None // TODO: the value at self.next (None past the end), then move next on by one
    }
}

#[cfg(test)]
#[allow(unused_imports, dead_code)]
mod tests {
    use super::*;
    use gpui::{TestAppContext, prelude::*};
    use std::{cell::Cell, rc::Rc};

    #[test]
    fn scripted_sensor_plays_in_order() {
        let mut cpu = ScriptedSensor::new(StatKind::CpuUsage, vec![1.0, 2.0, 3.0]);
        assert_eq!(cpu.kind(), StatKind::CpuUsage);
        assert_eq!(cpu.read(), Some(1.0));
        assert_eq!(cpu.read(), Some(2.0));
        assert_eq!(cpu.read(), Some(3.0));
        assert_eq!(cpu.read(), None);
    }
}
