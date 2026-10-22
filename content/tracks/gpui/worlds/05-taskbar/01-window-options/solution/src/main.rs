use gpui::{App, Application, Bounds, prelude::*, px, size};
use kinetik_05_window_options::*;

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(900.), px(48.)), cx);
        cx.open_window(bar_window_options(bounds), |_, cx| {
            cx.new(|cx| {
                let mut bar = StatsBar::new(live_sensors());
                bar.start(cx);
                bar
            })
        })
        .unwrap();
        cx.on_window_closed(|cx| cx.quit()).detach();
        cx.activate(true);
    });
}
