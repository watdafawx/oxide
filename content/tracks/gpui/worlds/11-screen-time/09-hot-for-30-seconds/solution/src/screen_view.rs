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

impl Render for ScreenTimeView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let day = self.screen.read(cx).day.clone();
        let summary = summarize(&day);
        let heights = bar_heights(&day.hours, CHART_HEIGHT);
        let or_dashes = |text: Option<String>| text.unwrap_or_else(|| "--".into());
        div()
            .debug_selector(|| "screen-time".into())
            .size_full()
            .p_4()
            .flex()
            .flex_col()
            .gap_2()
            .bg(rgb(0x15171c))
            .text_color(rgb(0xf2f2f2))
            .child(div().text_xl().child(format!("Screen time, {}", day.date)))
            .child(summary_line("total", "Total", hours_minutes(summary.total)))
            .child(summary_line("first", "First used", or_dashes(summary.first.map(clock))))
            .child(summary_line("last", "Last used", or_dashes(summary.last.map(clock))))
            .child(summary_line("most-used", "Most used", or_dashes(summary.most_used)))
            .child(summary_line("busiest", "Busiest hour", or_dashes(summary.busiest_hour.map(|hour| format!("{hour:02}:00")))))
            .child(
                div()
                    .debug_selector(|| "chart".into())
                    .flex_none()
                    .w_full()
                    .h(px(CHART_HEIGHT))
                    .flex()
                    .items_end()
                    .gap_1()
                    .children(heights.into_iter().enumerate().map(|(hour, height)| {
                        div().debug_selector(move || format!("hour-{hour}")).flex_1().h(px(height)).bg(rgb(0x5ab0ff))
                    })),
            )
    }
}
