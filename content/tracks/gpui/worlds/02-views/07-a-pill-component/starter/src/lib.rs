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

/// One pill: the label, then the value, in the value's colour.
#[derive(IntoElement)]
pub struct PillView {
    pub pill: Pill,
}

impl RenderOnce for PillView {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let kind = self.pill.kind;
        div()
            .debug_selector(move || format!("pill-{kind:?}"))
            .flex()
            .flex_row()
            .gap_1()
            // TODO: draw what pill_element draws: the named label box, the named
            // value box, and the value's text colour, from self.pill
    }
}

/// The old way: one pill as a plain function. Move what it draws into
/// PillView::render, then delete it.
#[allow(dead_code)]
fn pill_element(pill: &Pill) -> gpui::Div {
    let kind = pill.kind;
    div()
        .debug_selector(move || format!("pill-{kind:?}"))
        .flex()
        .flex_row()
        .gap_1()
        .text_color(value_colour(pill.warning))
        .child(div().debug_selector(move || format!("label-{kind:?}")).child(pill.label))
        .child(div().debug_selector(move || format!("value-{kind:?}")).child(pill.text.clone()))
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
            .children(self.pills.iter().map(|pill| PillView { pill: pill.clone() }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

    fn bounds(cx: &mut gpui::VisualTestContext, name: &'static str) -> gpui::Bounds<gpui::Pixels> {
        cx.debug_bounds(name).unwrap_or_else(|| panic!("nothing named {name} was drawn"))
    }

    #[gpui::test]
    fn pill_view_draws_label_then_value(cx: &mut TestAppContext) {
        let (_bar, cx) = cx.add_window_view(|_, _| StatsBar::demo());
        cx.run_until_parked();
        for kind in ["CpuUsage", "RamUsed", "Ping"] {
            let label = bounds(cx, Box::leak(format!("label-{kind}").into_boxed_str()));
            let value = bounds(cx, Box::leak(format!("value-{kind}").into_boxed_str()));
            assert!(value.origin.x > label.origin.x, "{kind}: the value sits right of the label");
        }
    }
}
