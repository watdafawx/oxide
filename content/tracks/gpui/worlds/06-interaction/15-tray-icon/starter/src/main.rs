use std::time::Duration;

use gpui::{App, Application, Bounds, TitlebarOptions, WindowBounds, WindowOptions, prelude::*, px, size};
use tray_icon::menu::{Menu, MenuEvent, MenuItem};
use tray_icon::{Icon, TrayIconBuilder, TrayIconEvent};
use kinetik_06_tray_icon::*;

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(900.), px(48.)), cx);
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: Some(TitlebarOptions { title: Some("Kinetik Stats".into()), ..Default::default() }),
            ..Default::default()
        };
        let window = cx
            .open_window(options, |window, cx| {
                cx.new(|cx| {
                    let mut bar = StatsBar::new(live_sensors());
                    bar.ui.top_processes = real_top_processes;
                    bar.focus_in(window, cx);
                    bar.start(cx);
                    bar
                })
            })
            .unwrap();

        // The tray icon: a blue square, a menu, and a loop that turns its events into actions.
        let menu = Menu::new();
        let _ = menu.append(&MenuItem::with_id("settings", "Settings…", true, None));
        let _ = menu.append(&MenuItem::with_id("exit", "Exit", true, None));
        let rgba: Vec<u8> = (0..16 * 16).flat_map(|_| [0x5a, 0xb0, 0xff, 0xff]).collect();
        let tray = TrayIconBuilder::new()
            .with_tooltip("Kinetik Stats")
            .with_icon(Icon::from_rgba(rgba, 16, 16).expect("a 16x16 icon"))
            .with_menu(Box::new(menu))
            .with_menu_on_left_click(false)
            .build()
            .ok();
        cx.spawn(async move |cx| {
            let _keep = tray;
            loop {
                cx.background_executor().timer(Duration::from_millis(100)).await;
                let mut wanted = Vec::new();
                while let Ok(event) = TrayIconEvent::receiver().try_recv() {
                    wanted.extend(tray_click_action(&event));
                }
                while let Ok(event) = MenuEvent::receiver().try_recv() {
                    wanted.extend(tray_menu_action(event.id().as_ref()));
                }
                for action in wanted {
                    let _ = window.update(cx, |_, window, cx| match action {
                        TrayAction::OpenSettings => window.dispatch_action(Box::new(OpenSettings), cx),
                        TrayAction::Exit => window.dispatch_action(Box::new(Exit), cx),
                    });
                }
            }
        })
        .detach();
        cx.on_window_closed(|cx| cx.quit()).detach();
        cx.activate(true);
    });
}
