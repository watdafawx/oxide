use gpui::{App, Application, Bounds, Pixels, TitlebarOptions, WindowBounds, WindowOptions, prelude::*, px, size};
use kinetik_02_a_pill_component::StatsBar;

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
        cx.on_window_closed(|cx| cx.quit()).detach();
        cx.activate(true);
    });
}
