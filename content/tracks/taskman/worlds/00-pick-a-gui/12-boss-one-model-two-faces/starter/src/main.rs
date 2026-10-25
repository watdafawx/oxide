//! `cargo run`: the GPUI face in a real window, with the sample processes.

use gpui::{App, Application, Bounds, TitlebarOptions, WindowBounds, WindowOptions, prelude::*, px, size};
use taskman_00_boss_one_model_two_faces::{TaskList, gpui_face::TaskListView, sample_processes};

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(480.), px(240.)), cx);
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: Some(TitlebarOptions { title: Some("Task manager".into()), ..Default::default() }),
            ..Default::default()
        };
        cx.open_window(options, |_, cx| cx.new(|_| TaskListView { list: TaskList::new(sample_processes()) }))
            .unwrap();
        cx.on_window_closed(|cx| cx.quit()).detach();
        cx.activate(true);
    });
}
