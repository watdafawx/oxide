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

use gpui::{AnyElement, App, Context, Div, IntoElement, Pixels, Render, RenderOnce, Rgba, Window, div, prelude::*, px, rgb};

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

/// The bar's rounded background: Kinetik's dark grey, see-through by `opacity` (0.0 to 1.0).
pub fn pill_background(opacity: f32) -> Rgba {
    let mut colour = rgb(0x202020);
    colour.a = opacity;
    colour
}

/// How wide each divider style is.
pub fn divider_width(style: DividerStyle) -> Pixels {
    match style {
        DividerStyle::Line => px(1.),
        DividerStyle::Dotted => px(1.),
        DividerStyle::Dot => px(4.),
        DividerStyle::Blank => px(8.),
    }
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
    pub two_lines: bool,
    pub opacity: f32,
}

impl StatsBar {
    /// The default bar, drawn from the demo snapshot.
    pub fn demo() -> Self {
        let items = bar_items(&default_order(), &demo::demo_snapshot(), &Limits::default());
        StatsBar { items, label_mode: LabelMode::Text, two_lines: false, opacity: 0.85 }
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

/// One column of the two-line layout: two pills stacked, one pill, or a divider.
fn column(index: usize, column: Column, show_label: bool) -> AnyElement {
    match column {
        Column::Pills(top, bottom) => div()
            .flex()
            .flex_col()
            .child(PillView { pill: top, show_label })
            .children(bottom.map(|pill| PillView { pill, show_label }))
            .into_any_element(),
        Column::Divider(style) => divider(index, style).into_any_element(),
    }
}

impl Render for StatsBar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let show_label = self.label_mode == LabelMode::Text;
        let bar = div()
            .debug_selector(|| "bar".into())
            .flex()
            .flex_row()
            .items_center()
            .gap_2()
            .h(px(32.))
            .px_2()
            .rounded(px(8.))
            .bg(pill_background(self.opacity))
            .text_color(rgb(0xffffff))
            .when(self.two_lines, |el| el.text_xs().line_height(px(15.)));
        if self.two_lines {
            bar.children(
                two_line_columns(&self.items)
                    .into_iter()
                    .enumerate()
                    .map(|(i, c)| column(i, c, show_label)),
            )
        } else {
            bar.children(self.items.iter().enumerate().map(|(i, item)| match item {
                BarItem::Pill(pill) => PillView { pill: pill.clone(), show_label }.into_any_element(),
                BarItem::Divider(style) => divider(i, *style).into_any_element(),
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opacity_50_sets_half_alpha() {
        let c = pill_background(0.5);
        assert_eq!(c.a, 0.5);
        assert_eq!((c.r, c.g, c.b), (rgb(0x202020).r, rgb(0x202020).g, rgb(0x202020).b), "still the dark grey");
    }

    #[test]
    fn full_opacity_is_solid() {
        assert_eq!(pill_background(1.0), rgb(0x202020));
    }
}
