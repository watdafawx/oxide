use gpui::{App, Application, prelude::*};
use kinetik_05_light_or_dark::*;

/// The bar, docked on the primary taskbar. If the taskbar is auto-hidden, it opens above where it would be.
fn main() {
    Application::new().run(|cx: &mut App| {
        let taskbar = find_taskbar().expect("no taskbar found");
        let place = dock_bar(&taskbar, 600, 0).unwrap_or_else(|| {
            println!("The taskbar is hidden; showing the bar just above it instead.");
            let bottom = taskbar.monitor.bottom - 60;
            Rect::new(taskbar.tray.left - 608, bottom - 40, taskbar.tray.left - 8, bottom)
        });
        let bounds = to_logical(place, taskbar.scale);
        cx.open_window(bar_window_options(bounds), |_, cx| {
            cx.new(|cx| {
                let mut bar = StatsBar::new(live_sensors());
                bar.start(cx);
                bar
            })
        })
        .unwrap();
        cx.activate(false);
    });
}
