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

use gpui::{App, Context, Div, IntoElement, Pixels, Render, RenderOnce, Rgba, Window, div, prelude::*, px, rgb};

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

/// How wide each divider style is.
pub fn divider_width(style: DividerStyle) -> Pixels {
    let _ = style;
    px(1.) // TODO: a width per style: 1 px for both lines, 4 for a dot, 8 for blank space
}

/// A divider: a thin line, a dashed line, a dot, or a gap.
fn divider(index: usize, style: DividerStyle) -> Div {
    let grey = rgb(0x5a5a5a);
    let el = div().debug_selector(move || format!("divider-{index}")).flex_none().w(divider_width(style));
    match style {
        DividerStyle::Line => el.h(px(16.)).bg(grey),
        DividerStyle::Dotted => el.h(px(16.)).border_l_1().border_dashed().border_color(grey),
        DividerStyle::Dot => el.h(px(4.)).rounded_full().bg(grey),
        DividerStyle::Blank => el,
    }
}

/// The bar of stats: pills and dividers in the default order.
pub struct StatsBar {
    pub items: Vec<BarItem>,
    pub label_mode: LabelMode,
}

impl StatsBar {
    /// The default bar, drawn from the demo snapshot.
    pub fn demo() -> Self {
        let items = bar_items(&default_order(), &demo::demo_snapshot(), &Limits::default());
        StatsBar { items, label_mode: LabelMode::Text }
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
            .children(self.items.iter().enumerate().map(|(i, item)| match item {
                BarItem::Pill(pill) => PillView { pill: pill.clone(), show_label }.into_any_element(),
                BarItem::Divider(style) => divider(i, *style).into_any_element(),
            }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

    fn bounds(cx: &mut gpui::VisualTestContext, name: &'static str) -> gpui::Bounds<gpui::Pixels> {
        cx.debug_bounds(name).unwrap_or_else(|| panic!("nothing named {name} was drawn"))
    }

    fn bar_with(dividers: [DividerStyle; 2]) -> StatsBar {
        let p = |kind: StatKind| BarItem::Pill(Pill { kind, label: kind.label(), text: "1%".into(), warning: false });
        StatsBar {
            items: vec![
                p(StatKind::CpuUsage),
                BarItem::Divider(dividers[0]),
                p(StatKind::RamUsed),
                BarItem::Divider(dividers[1]),
                p(StatKind::Ping),
            ],
            label_mode: LabelMode::Text,
        }
    }

    #[gpui::test]
    fn dot_divider_is_4_wide(cx: &mut TestAppContext) {
        let (_bar, cx) = cx.add_window_view(|_, _| bar_with([DividerStyle::Dot, DividerStyle::Line]));
        cx.run_until_parked();
        assert_eq!(bounds(cx, "divider-1").size.width, px(4.));
    }

    #[gpui::test]
    fn blank_divider_is_8_wide(cx: &mut TestAppContext) {
        let (_bar, cx) = cx.add_window_view(|_, _| bar_with([DividerStyle::Line, DividerStyle::Blank]));
        cx.run_until_parked();
        assert_eq!(bounds(cx, "divider-3").size.width, px(8.));
        assert_eq!(bounds(cx, "divider-1").size.width, px(1.));
    }
}
