use gpui::{
    App, Application, Bounds, TitlebarOptions, WindowBounds, WindowOptions, prelude::*,
};
use kinetik_00_a_view::{app_title, bar_size, StatsBar};

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, bar_size(), cx);
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: Some(TitlebarOptions { title: Some(app_title().into()), ..Default::default() }),
            ..Default::default()
        };
        cx.open_window(options, |_, cx| cx.new(|_| StatsBar)).unwrap();
        cx.on_window_closed(|cx| cx.quit()).detach();
        cx.activate(true);
    });
}
