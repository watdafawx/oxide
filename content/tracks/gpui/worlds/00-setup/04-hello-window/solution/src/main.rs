use gpui::{
    App, Application, Bounds, Context, TitlebarOptions, Window, WindowBounds, WindowOptions, div,
    prelude::*,
};
use kinetik_00_hello_window::{app_title, bar_size};

/// Nothing inside yet: the next quest fills the window.
struct Empty;

impl Render for Empty {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, bar_size(), cx);
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: Some(TitlebarOptions { title: Some(app_title().into()), ..Default::default() }),
            ..Default::default()
        };
        cx.open_window(options, |_, cx| cx.new(|_| Empty)).unwrap();
        cx.on_window_closed(|cx| cx.quit()).detach();
        cx.activate(true);
    });
}
