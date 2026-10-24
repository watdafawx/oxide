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

/// The bar of stats, holding its own state.
pub struct StatsBar {
    pub snapshot: Snapshot,
    pub items: Vec<BarItem>,
    pub look: Look,
}

impl StatsBar {
    /// A bar showing `snapshot` in the default order.
    pub fn new(snapshot: Snapshot) -> Self {
        // TODO: the items the bar draws, made from this snapshot with the default order and limits
        let items = Vec::new();
        StatsBar { snapshot, items, look: Look::default() }
    }

    /// The text the pill for `kind` shows, if it is on the bar.
    pub fn text_of(&self, kind: StatKind) -> Option<String> {
        self.items.iter().find_map(|item| match item {
            BarItem::Pill(pill) if pill.kind == kind => Some(pill.text.clone()),
            _ => None,
        })
    }
}

#[cfg(test)]
#[allow(unused_imports, dead_code)]
mod tests {
    use super::*;
    use gpui::{TestAppContext, prelude::*};
    use std::{cell::Cell, rc::Rc};

    #[gpui::test]
    fn bar_shows_its_snapshot(cx: &mut TestAppContext) {
        let snapshot = Snapshot::from([(StatKind::CpuUsage, 42.0)]);
        let (bar, cx) = cx.add_window_view(move |_, _| StatsBar::new(snapshot));
        cx.run_until_parked();
        assert!(cx.debug_bounds("pill-CpuUsage").is_some(), "the CPU pill is drawn");
        assert_eq!(bar.read_with(cx, |bar, _| bar.text_of(StatKind::CpuUsage)), Some("42%".to_string()));
    }
}
