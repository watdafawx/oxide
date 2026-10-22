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
use gpui::{App, AppContext, Entity};

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
}

/// Hand a new bar to GPUI and get its handle back.
pub fn new_bar(cx: &mut App, snapshot: Snapshot) -> Entity<StatsBar> {
    let _ = (cx, snapshot);
    todo!("cx.new with a closure that makes StatsBar::new(snapshot)")
}

#[cfg(test)]
#[allow(unused_imports, dead_code)]
mod tests {
    use super::*;
    use gpui::{TestAppContext, prelude::*};
    use std::{cell::Cell, rc::Rc};

    #[gpui::test]
    fn entity_holds_bar_state(cx: &mut TestAppContext) {
        let snapshot = Snapshot::from([(StatKind::CpuUsage, 42.0)]);
        let bar = cx.update(|cx| new_bar(cx, snapshot));
        let text = bar.read_with(cx, |bar, _| bar.text_of(StatKind::CpuUsage));
        assert_eq!(text, Some("42%".to_string()));
    }
}
