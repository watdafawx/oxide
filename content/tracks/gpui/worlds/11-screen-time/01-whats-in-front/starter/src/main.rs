use std::sync::{Arc, Mutex};

use gpui::{App, Application, Bounds, TitlebarOptions, UpdateGlobal, WindowBounds, WindowOptions, prelude::*, px, size};
use kinetik_11_whats_in_front::*;

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.set_global(load(&settings_path()));
        bind_settings_keys(cx);
        persist_settings(settings_path(), cx).detach();
        // The web dashboard: a key on first start, then served on its own thread.
        if cx.global::<Settings>().dashboard.key.is_empty() {
            Settings::update_global(cx, |s, _| s.dashboard.key = new_key());
            // Saved by hand: the observer above only starts listening after this startup code returns.
            let _ = save(cx.global::<Settings>(), &settings_path());
        }
        let shared = Arc::new(Mutex::new(WebSnapshot::default()));
        let dash = cx.global::<Settings>().dashboard.clone();
        let addr = bind_addr(dash.share_on_lan, dash.port);
        println!("Web dashboard: http://{addr}/?key={}", dash.key);
        let _server = serve_dashboard(addr, WebState { snapshot: shared.clone(), key: dash.key.clone() });
        // Screen time, first look: print the program in front every 5 seconds.
        std::thread::spawn(|| {
            loop {
                std::thread::sleep(std::time::Duration::from_secs(5));
                println!("In front: {:?}", app_to_count(&RealDesk));
            }
        });
        let bounds = Bounds::centered(None, size(px(900.), px(48.)), cx);
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: Some(TitlebarOptions { title: Some("Kinetik Stats".into()), ..Default::default() }),
            ..Default::default()
        };
        cx.open_window(options, |window, cx| {
            cx.new(|cx| {
                let mut bar = StatsBar::from_settings(live_sensors(), cx);
                bar.ui.top_processes = real_top_processes;
                bar.focus_in(window, cx);
                bar.watch_settings(cx);
                bar.start(cx);
                let shared = shared.clone();
                cx.observe_self(move |bar, _| *shared.lock().unwrap_or_else(|p| p.into_inner()) = web_snapshot(bar)).detach();
                bar.toggle_dashboard(cx);
                bar
            })
        })
        .unwrap();
        cx.on_window_closed(|cx| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();
        cx.activate(true);
    });
}
