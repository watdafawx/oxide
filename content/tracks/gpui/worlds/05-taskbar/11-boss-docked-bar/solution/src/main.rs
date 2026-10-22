use std::time::Duration;

use gpui::{App, Application, prelude::*};
use kinetik_05_boss_docked_bar::*;

/// Kinetik Stats on the taskbar: one bar per taskbar, docked, on top, hidden in fullscreen.
fn main() {
    Application::new().run(|cx: &mut App| {
        let all_taskbars = false;
        for taskbar in taskbars_to_use(&RealWin.taskbars(), all_taskbars) {
            let place = dock_bar(&taskbar, 600, 0).unwrap_or_else(|| {
                println!("A taskbar is hidden; its bar appears when it slides up.");
                Rect::new(taskbar.tray.left - 608, taskbar.monitor.bottom - 48, taskbar.tray.left - 8, taskbar.monitor.bottom)
            });
            let options = bar_window_options(to_logical(place, taskbar.scale));
            cx.open_window(options, move |window, cx| {
                let hwnd = hwnd_of(window).unwrap_or_default();
                cx.new(|cx| {
                    let mut bar = StatsBar::new(live_sensors());
                    bar.start(cx);
                    let mut docker = Docker::new(RealWin, hwnd, 600);
                    docker.on_open();
                    cx.spawn(async move |_, cx| {
                        loop {
                            cx.background_executor().timer(Duration::from_secs(1)).await;
                            docker.tick();
                        }
                    })
                    .detach();
                    bar
                })
            })
            .unwrap();
        }
        cx.activate(false);
    });
}
