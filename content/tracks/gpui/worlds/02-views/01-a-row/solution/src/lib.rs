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

use gpui::{Context, IntoElement, Render, Window, div, prelude::*};

/// The bar of stats. Two fixed pills for now.
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
            .gap_2()
            .child(div().debug_selector(|| "pill-cpu".into()).child("CPU 13%"))
            .child(div().debug_selector(|| "pill-ram".into()).child("RAM 8.0 GB"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

    #[gpui::test]
    fn ram_pill_is_right_of_cpu_pill(cx: &mut TestAppContext) {
        let (_bar, cx) = cx.add_window_view(|_, _| StatsBar::demo());
        cx.run_until_parked();
        let cpu = cx.debug_bounds("pill-cpu").expect("cpu pill drawn");
        let ram = cx.debug_bounds("pill-ram").expect("ram pill drawn");
        assert_eq!(ram.origin.y, cpu.origin.y, "same line");
        assert!(ram.origin.x > cpu.origin.x, "RAM is right of CPU: cpu {cpu:?}, ram {ram:?}");
    }
}
