use std::sync::{Arc, Mutex, atomic::{AtomicU32, Ordering}};
use std::time::{Duration, Instant};

use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager};
use gpui::{App, Application, Bounds, TitlebarOptions, WindowBounds, WindowOptions, prelude::*, px, size};
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId};
use kinetik_09_boss_game_overlay::*;

/// The process in front, and whether it is a game.
fn front() -> Front {
    // SAFETY: plain window queries.
    let pid = unsafe {
        let hwnd = GetForegroundWindow();
        let mut pid = 0u32;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));
        pid
    };
    match RealWin.foreground() {
        Some((window, monitor)) if covers_monitor(window, monitor) => Front::Game(pid),
        _ => Front::Other,
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.set_global(load(&settings_path()));
        bind_settings_keys(cx);
        persist_settings(settings_path(), cx).detach();

        // Live frames for whatever is in front, if Windows lets us listen.
        let feed: FrameFeed = Arc::new(Mutex::new(FrameWindow::default()));
        let target = Arc::new(AtomicU32::new(0));
        let last = Arc::new(Mutex::new(None::<f64>));
        let (sink, wanted, previous) = (feed.clone(), target.clone(), last.clone());
        let trace = start_present_trace(move |pid, time| {
            if pid != wanted.load(Ordering::Relaxed) {
                return;
            }
            let mut previous = previous.lock().unwrap_or_else(|p| p.into_inner());
            if let Some(before) = previous.replace(time) {
                let frame = Frame { time, ms: (time - before) * 1000.0, generated: false };
                sink.lock().unwrap_or_else(|p| p.into_inner()).push(frame);
            }
        });
        let reason = trace.as_ref().err().cloned();

        let hotkeys = GlobalHotKeyManager::new().ok();
        if let Some(manager) = &hotkeys {
            let _ = manager.register(overlay_hotkey());
            let _ = manager.register(benchmark_hotkey());
        }

        let bounds = Bounds::centered(None, size(px(900.), px(48.)), cx);
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: Some(TitlebarOptions { title: Some("Kinetik Stats".into()), ..Default::default() }),
            ..Default::default()
        };
        let bar_window = cx
            .open_window(options, |window, cx| {
                cx.new(|cx| {
                    let mut bar = StatsBar::from_settings(live_sensors(), cx);
                    bar.ui.top_processes = real_top_processes;
                    bar.ui.frame_feed = feed.clone();
                    bar.ui.live_reason = reason.clone();
                    bar.focus_in(window, cx);
                    bar.watch_settings(cx);
                    bar.start(cx);
                    bar
                })
            })
            .unwrap();

        let started = Instant::now();
        cx.spawn(async move |cx| {
            let _keep = (trace, hotkeys);
            let mut controller = OverlayController::default();
            loop {
                cx.background_executor().timer(Duration::from_millis(250)).await;
                while let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
                    match hotkey_action(&event) {
                        Some(HotkeyAction::ToggleOverlay) => {
                            let _ = bar_window.update(cx, |_, window, cx| window.dispatch_action(Box::new(ToggleOverlay), cx));
                        }
                        Some(HotkeyAction::ToggleBenchmark) => controller.benchmark.toggle(),
                        None => {}
                    }
                }
                let now = started.elapsed().as_secs_f64();
                let front = front();
                if let Front::Game(pid) = front {
                    target.store(pid, Ordering::Relaxed);
                }
                let frames: Vec<Frame> = feed.lock().unwrap_or_else(|p| p.into_inner()).frames.iter().copied().collect();
                let stats = FrameStats::compute(&frames);
                let Ok((snapshot, gpu)) = bar_window.read_with(cx, |bar, _| (bar.snapshot.clone(), bar.snapshot.get(&StatKind::GpuTemp).copied())) else {
                    break;
                };
                controller.benchmark.tick(&snapshot, (!frames.is_empty()).then_some(stats.fps));
                let running = |pid: u32| {
                    let pid = sysinfo::Pid::from_u32(pid);
                    let mut system = sysinfo::System::new();
                    system.refresh_processes(sysinfo::ProcessesToUpdate::Some(&[pid]), true);
                    system.process(pid).is_some()
                };
                let step = controller.tick(now, front, "game", running, &stats, gpu);
                if matches!(step, Some(OverlayStep::Show) | Some(OverlayStep::Close)) {
                    let _ = bar_window.update(cx, |_, window, cx| window.dispatch_action(Box::new(ToggleOverlay), cx));
                }
            }
        })
        .detach();
        cx.on_window_closed(|cx| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();
        cx.activate(true);
    });
}
