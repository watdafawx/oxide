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

use gpui::{Context, IntoElement, Render, Rgba, Window, div, prelude::*, px, rgb};

/// Kinetik's value colours: white, or the warning red.
pub fn value_colour(warning: bool) -> Rgba {
    let _ = warning;
    rgb(0x808080) // TODO: Kinetik's warning red (ff5c5c) when warning, white otherwise
}

/// The bar of stats. Three fixed pills for now.
pub struct StatsBar;

impl StatsBar {
    pub fn demo() -> Self {
        StatsBar
    }
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
            .child(div().text_color(value_colour(false)).child("CPU 13%"))
            .child(div().text_color(value_colour(true)).child("CPU 88°C"))
            .child(div().text_color(value_colour(false)).child("RAM 8.0 GB"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn warning_colour_is_kinetik_red() {
        assert_eq!(value_colour(true), rgb(0xff5c5c));
    }

    #[test]
    fn normal_colour_is_white() {
        assert_eq!(value_colour(false), rgb(0xffffff));
    }
}
