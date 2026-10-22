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
    fn new_reading_redraws_bar(cx: &mut TestAppContext) {
        let bar = cx.new(|_| StatsBar::new(Snapshot::new()));
        let notifies = count_notifies(cx, &bar);
        bar.update(cx, |bar, cx| bar.set_reading(StatKind::CpuUsage, 42.0, cx));
        cx.run_until_parked();
        assert_eq!(bar.read_with(cx, |bar, _| bar.text_of(StatKind::CpuUsage)), Some("42%".to_string()));
        assert_eq!(notifies.get(), 1, "set_reading should call cx.notify() once");
    }
}
