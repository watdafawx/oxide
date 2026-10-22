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

use gpui::{App, Context, IntoElement, Render, RenderOnce, Rgba, Window, div, prelude::*, px, rgb};

/// Kinetik's value colours: white, or the warning red.
pub fn value_colour(warning: bool) -> Rgba {
    if warning { rgb(0xff5c5c) } else { rgb(0xffffff) }
}

/// Kinetik's icon picker, cut down: a short text label, or nothing in front of the value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelMode {
    Text,
    Off,
}

/// The bar of stats, one pill per stat in the default order.
pub struct StatsBar {
    pub pills: Vec<Pill>,
    pub label_mode: LabelMode,
}

impl StatsBar {
    /// The default bar order, drawn from the demo snapshot, with text labels.
    pub fn demo() -> Self {
        let items = bar_items(&default_order(), &demo::demo_snapshot(), &Limits::default());
        let pills = items
            .into_iter()
            .filter_map(|item| match item {
                BarItem::Pill(pill) => Some(pill),
                BarItem::Divider(_) => None,
            })
            .collect();
        StatsBar { pills, label_mode: LabelMode::Text }
    }
}

/// One pill: the label (unless labels are off), then the value.
#[derive(IntoElement)]
pub struct PillView {
    pub pill: Pill,
    pub show_label: bool,
}

impl RenderOnce for PillView {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let kind = self.pill.kind;
        let label = self.pill.label;
        div()
            .debug_selector(move || format!("pill-{kind:?}"))
            .flex()
            .flex_row()
            .gap_1()
            .when(self.show_label, |el| {
                el.child(div().debug_selector(move || format!("label-{kind:?}")).child(label))
            })
            .child(div().debug_selector(move || format!("value-{kind:?}")).child(self.pill.text))
            .text_color(value_colour(self.pill.warning))
    }
}

impl Render for StatsBar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let show_label = self.label_mode == LabelMode::Text;
        div()
            .debug_selector(|| "bar".into())
            .flex()
            .flex_row()
            .items_center()
            .gap_2()
            .h(px(32.))
            .bg(rgb(0x202020))
            .children(self.pills.iter().map(|pill| PillView { pill: pill.clone(), show_label }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

    #[gpui::test]
    fn text_mode_draws_label(cx: &mut TestAppContext) {
        let (_bar, cx) = cx.add_window_view(|_, _| StatsBar::demo());
        cx.run_until_parked();
        assert!(cx.debug_bounds("label-CpuUsage").is_some());
    }

    #[gpui::test]
    fn off_mode_draws_no_label(cx: &mut TestAppContext) {
        let (_bar, cx) = cx.add_window_view(|_, _| StatsBar { label_mode: LabelMode::Off, ..StatsBar::demo() });
        cx.run_until_parked();
        assert!(cx.debug_bounds("label-CpuUsage").is_none(), "labels are off, but label-CpuUsage was drawn");
        assert!(cx.debug_bounds("value-CpuUsage").is_some(), "the value is still drawn");
    }
}
