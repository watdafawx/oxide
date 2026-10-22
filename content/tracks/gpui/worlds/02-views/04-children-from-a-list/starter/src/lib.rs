//! Kinetik Stats: the model, and the bar drawn with GPUI.

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

use gpui::{Context, Div, IntoElement, Render, Rgba, Window, div, prelude::*, px, rgb};

/// Kinetik's value colours: white, or the warning red.
pub fn value_colour(warning: bool) -> Rgba {
    if warning { rgb(0xff5c5c) } else { rgb(0xffffff) }
}

/// The bar of stats, one pill per stat in the default order.
pub struct StatsBar {
    pub pills: Vec<Pill>,
}

impl StatsBar {
    /// The default bar order, drawn from the demo snapshot. Dividers come later.
    pub fn demo() -> Self {
        let items = bar_items(&default_order(), &demo::demo_snapshot(), &Limits::default());
        let pills = items
            .into_iter()
            .filter_map(|item| match item {
                BarItem::Pill(pill) => Some(pill),
                BarItem::Divider(_) => None,
            })
            .collect();
        StatsBar { pills }
    }
}

/// One pill: label and value, in the value's colour.
fn pill_element(pill: &Pill) -> Div {
    let kind = pill.kind;
    div()
        .debug_selector(move || format!("pill-{kind:?}"))
        .flex()
        .flex_row()
        .gap_1()
        .text_color(value_colour(pill.warning))
        .child(pill.label)
        .child(pill.text.clone())
}

impl Render for StatsBar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .debug_selector(|| "bar".into())
            .flex()
            .flex_row()
            .items_center()
            .gap_2()
            .h(px(32.))
            .bg(rgb(0x202020))
            .child(pill_element(&self.pills[0])) // TODO: one pill_element per pill: .children(...)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

    #[gpui::test]
    fn one_element_per_pill(cx: &mut TestAppContext) {
        let (_bar, cx) = cx.add_window_view(|_, _| StatsBar::demo());
        cx.run_until_parked();
        for name in [
            "pill-CpuUsage", "pill-CpuTemp", "pill-GpuUsage", "pill-GpuTemp",
            "pill-RamUsed", "pill-NetDown", "pill-NetUp", "pill-Ping",
        ] {
            assert!(cx.debug_bounds(name).is_some(), "{name} was not drawn");
        }
    }
}
