//! The screen time page: today's summary and a chart of the hours.

#![allow(unused_imports)]

use gpui::{App, Bounds, Context, Div, Entity, IntoElement, Render, Subscription, TitlebarOptions, Window, WindowBounds, WindowHandle, WindowOptions, actions, div,
           prelude::*, px, rgb, size};

use crate::*;

actions!(screen_time, [ToggleScreenTime]);

/// How tall the busiest hour's bar is drawn.
pub const CHART_HEIGHT: f32 = 160.0;

/// The screen time page.
pub struct ScreenTimeView {
    pub screen: Entity<ScreenTime>,
    _watch: Subscription,
}

impl ScreenTimeView {
    pub fn new(screen: Entity<ScreenTime>, cx: &mut Context<Self>) -> Self {
        let watch = cx.observe(&screen, |_, _, cx| cx.notify());
        ScreenTimeView { screen, _watch: watch }
    }
}

/// Open the screen time page, or close it if it is open.
pub fn toggle_screen_window(cx: &mut App, slot: &mut Option<WindowHandle<ScreenTimeView>>, screen: Entity<ScreenTime>) {
    if let Some(handle) = slot.take()
        && handle.update(cx, |_, window, _| window.remove_window()).is_ok()
    {
        return;
    }
    let bounds = Bounds::centered(None, size(px(720.), px(480.)), cx);
    let options = WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(bounds)),
        titlebar: Some(TitlebarOptions { title: Some("Kinetik Stats Screen Time".into()), ..Default::default() }),
        ..Default::default()
    };
    *slot = cx.open_window(options, |_, cx| cx.new(|cx| ScreenTimeView::new(screen, cx))).ok();
}

impl StatsBar {
    /// Open or close the screen time page.
    pub fn toggle_screen_time(&mut self, cx: &mut Context<Self>) {
        let Some(screen) = self.ui.screen_time.clone() else {
            return;
        };
        toggle_screen_window(cx, &mut self.ui.screen_window, screen);
    }
}

/// One line of the summary: a grey label, then its value.
pub fn summary_line(name: &'static str, label: &str, value: String) -> Div {
    div()
        .debug_selector(move || name.to_string())
        .flex()
        .gap_2()
        .child(div().w(px(120.)).text_color(rgb(0x9a9a9a)).child(label.to_string()))
        .child(value)
}
