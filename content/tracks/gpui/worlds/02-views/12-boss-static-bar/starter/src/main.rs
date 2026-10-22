use gpui::{App, Application, Bounds, Pixels, TitlebarOptions, WindowBounds, WindowOptions, point, prelude::*, px, size};
use kinetik_02_boss_static_bar::StatsBar;

fn options(bounds: Bounds<Pixels>, title: &str) -> WindowOptions {
    WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(bounds)),
        titlebar: Some(TitlebarOptions { title: Some(title.to_string().into()), ..Default::default() }),
        ..Default::default()
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(820.), px(48.)), cx);
        cx.open_window(options(bounds, "Kinetik Stats"), |_, cx| cx.new(|_| StatsBar::demo())).unwrap();
        let two = Bounds::new(point(bounds.origin.x, bounds.origin.y + px(80.)), bounds.size);
        cx.open_window(options(two, "Kinetik Stats: two lines"), |_, cx| {
            cx.new(|_| StatsBar { two_lines: true, ..StatsBar::demo() })
        })
        .unwrap();
        cx.on_window_closed(|cx| cx.quit()).detach();
        cx.activate(true);
    });
}
