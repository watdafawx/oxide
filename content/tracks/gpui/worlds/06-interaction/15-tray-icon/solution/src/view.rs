//! How the bar looks: the drawing code from the Views world.

use gpui::{AnyElement, Context, Div, IntoElement, Pixels, Render, Rgba, Window, WindowAppearance, div, prelude::*, px, rgb};

use crate::{bar_layer, interactive_pill};

use crate::StatsBar;
use crate::order::DividerStyle;
use crate::pills::{BarItem, Column, Pill, two_line_columns};

/// Kinetik's icon picker, cut down: a short text label, or nothing in front of the value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelMode {
    Text,
    Off,
}

/// The Appearance page's settings for the bar.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Look {
    pub label_mode: LabelMode,
    pub two_lines: bool,
    pub opacity: f32,
    pub font_size: f32,
}

impl Default for Look {
    fn default() -> Self {
        Look { label_mode: LabelMode::Text, two_lines: false, opacity: 0.85, font_size: 13.0 }
    }
}

/// Text on the taskbar: white on a dark taskbar, near-black on a light one.
pub fn text_colour_for(appearance: WindowAppearance) -> Rgba {
    match appearance {
        WindowAppearance::Dark | WindowAppearance::VibrantDark => rgb(0xffffff),
        WindowAppearance::Light | WindowAppearance::VibrantLight => rgb(0x1a1a1a),
    }
}

/// Kinetik's value colours: white, or the warning red.
pub fn value_colour(warning: bool) -> Rgba {
    if warning { rgb(0xff5c5c) } else { rgb(0xffffff) }
}

/// The bar's rounded background: dark grey, see-through by `opacity`.
pub fn pill_background(opacity: f32) -> Rgba {
    let mut colour = rgb(0x202020);
    colour.a = opacity;
    colour
}

/// The bar's text size, kept between 8 and 24 pixels.
pub fn bar_text_size(setting: f32) -> Pixels {
    px(setting.clamp(8.0, 24.0))
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

fn pill_element(pill: Pill, show_label: bool, bar: &StatsBar, cx: &mut Context<StatsBar>) -> AnyElement {
    let kind = pill.kind;
    let label = pill.label;
    let el = div()
        .debug_selector(move || format!("pill-{kind:?}"))
        .flex()
        .flex_row()
        .gap_1()
        .when(show_label, |el| el.child(div().debug_selector(move || format!("label-{kind:?}")).child(label)))
        .child(
            div()
                .debug_selector(move || format!("value-{kind:?}"))
                .min_w(px(48.))
                .text_color(value_colour(pill.warning))
                .child(pill.text),
        );
    interactive_pill(el, kind, bar, cx).into_any_element()
}

fn column(index: usize, column: Column, show_label: bool, bar: &StatsBar, cx: &mut Context<StatsBar>) -> AnyElement {
    match column {
        Column::Pills(top, bottom) => div()
            .flex()
            .flex_col()
            .child(pill_element(top, show_label, bar, cx))
            .children(bottom.map(|pill| pill_element(pill, show_label, bar, cx)))
            .into_any_element(),
        Column::Divider(style) => divider(index, style).into_any_element(),
    }
}

impl Render for StatsBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let look = self.look;
        let show_label = look.label_mode == LabelMode::Text;
        let bar = div()
            .debug_selector(|| "bar".into())
            .flex()
            .flex_row()
            .items_center()
            .gap_2()
            .h(px(32.))
            .px_2()
            .rounded(px(8.))
            .bg(pill_background(look.opacity))
            .text_color(text_colour_for(window.appearance()))
            .font_family("Consolas")
            .text_size(bar_text_size(look.font_size))
            .when(look.two_lines, |el| el.text_xs().line_height(px(15.)));
        let bar = bar_layer(bar, self, cx);
        let this = &*self;
        if look.two_lines {
            bar.children(
                two_line_columns(&self.items)
                    .into_iter()
                    .enumerate()
                    .map(|(i, c)| column(i, c, show_label, this, cx)),
            )
        } else {
            bar.children(self.items.iter().enumerate().map(|(i, item)| match item {
                BarItem::Pill(pill) => pill_element(pill.clone(), show_label, this, cx),
                BarItem::Divider(style) => divider(i, *style).into_any_element(),
            }))
        }
    }
}
