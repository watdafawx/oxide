//! Kinetik Stats, rebuilt in Rust.

use gpui::{Context, IntoElement, Pixels, Render, Size, Window, div, prelude::*, px, size};

/// The name on Kinetik's window.
pub fn app_title() -> String {
    "Kinetik Stats".to_string()
}

/// How big Kinetik's first window is.
pub fn bar_size() -> Size<Pixels> {
    size(px(360.), px(120.))
}

/// The bar of stats. One pill for now.
pub struct StatsBar;

impl Render for StatsBar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        // TODO: give this box one child: a box named "pill-cpu" that holds the text "CPU 12%"
        div()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

    #[gpui::test]
    fn bar_draws_a_cpu_pill(cx: &mut TestAppContext) {
        let (_bar, cx) = cx.add_window_view(|_, _| StatsBar);
        cx.run_until_parked();
        assert!(cx.debug_bounds("pill-cpu").is_some(), "no element named pill-cpu was drawn");
    }
}
