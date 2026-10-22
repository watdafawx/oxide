//! The game overlay: games, the strip window, hotkeys, auto-show, sessions and benchmarks.

#![allow(unused_imports)]

use std::sync::{Arc, Mutex};
use std::time::Duration;

use global_hotkey::GlobalHotKeyEvent;
use global_hotkey::HotKeyState;
use global_hotkey::hotkey::{Code, HotKey, Modifiers};
use gpui::{App, Bounds, Context, Div, IntoElement, PathBuilder, Pixels, Render, Task, Window, WindowBackgroundAppearance, WindowBounds, WindowHandle, WindowKind, WindowOptions, actions, canvas, div, point, prelude::*, px, rgb, rgba, size};

use crate::*;

/// Programs that fill the screen without being games.
pub const NOT_GAMES: [&str; 3] = ["explorer.exe", "LockApp.exe", "ShellExperienceHost.exe"];

/// A game, for the overlay: the window in front covers its monitor (fullscreen or borderless),
/// and it is not Windows' own desktop or lock screen.
pub fn is_game(window: Rect, monitor: Rect, exe: &str) -> bool {
    covers_monitor(window, monitor) && !NOT_GAMES.iter().any(|name| exe.eq_ignore_ascii_case(name))
}

/// The overlay strip's place: centred at the top of the screen, 8 pixels down.
pub fn overlay_bounds(screen: Bounds<Pixels>, width: Pixels, height: Pixels) -> Bounds<Pixels> {
    let x = screen.origin.x + (screen.size.width - width) / 2.0;
    Bounds::new(point(x, screen.origin.y + px(8.)), size(width, height))
}

/// Ctrl+Shift+F10 shows or hides the overlay, even inside a game.
pub fn overlay_hotkey() -> HotKey {
    HotKey::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::F10)
}

/// Ctrl+Shift+F11 starts and stops benchmark recording.
pub fn benchmark_hotkey() -> HotKey {
    HotKey::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::F11)
}

/// What a global hotkey asks for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HotkeyAction {
    ToggleOverlay,
    ToggleBenchmark,
}

/// A hotkey event to an action: only on the press, never on the release.
pub fn hotkey_action(event: &GlobalHotKeyEvent) -> Option<HotkeyAction> {
    if event.state != HotKeyState::Pressed {
        return None;
    }
    if event.id == overlay_hotkey().id() {
        Some(HotkeyAction::ToggleOverlay)
    } else if event.id == benchmark_hotkey().id() {
        Some(HotkeyAction::ToggleBenchmark)
    } else {
        None
    }
}

/// What is in front, as the overlay sees it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Front {
    /// A game, by process id.
    Game(u32),
    /// Something else.
    Other,
}

/// What the overlay should do now.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverlayStep {
    Show,
    Hide,
    /// The game ended: close the overlay and write its session summary.
    Close,
}

/// "Show automatically in games": remembers the game it is showing for.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AutoShow {
    pub game: Option<u32>,
    pub shown: bool,
}

impl AutoShow {
    /// One check: what is in front, and whether a process is still running.
    pub fn step(&mut self, front: Front, running: impl Fn(u32) -> bool) -> Option<OverlayStep> {
        if let Some(game) = self.game
            && !running(game)
        {
            *self = AutoShow::default();
            return Some(OverlayStep::Close);
        }
        match front {
            Front::Game(pid) if !self.shown => {
                self.game = Some(pid);
                self.shown = true;
                Some(OverlayStep::Show)
            }
            Front::Other if self.shown => {
                self.shown = false;
                Some(OverlayStep::Hide)
            }
            _ => None,
        }
    }
}

/// What the overlay shows, worked out from a window of frames.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FrameStats {
    pub fps: f64,
    pub base_fps: f64,
    pub one_percent_low: f64,
    pub point_one_percent_low: f64,
    /// The last 3 seconds of frame times, for the graph.
    pub recent_ms: Vec<f64>,
    /// Positions in `recent_ms` that are stutters.
    pub spikes: Vec<usize>,
}

impl FrameStats {
    /// Everything at once, from the frames in the window.
    pub fn compute(frames: &[Frame]) -> FrameStats {
        let ms: Vec<f64> = frames.iter().map(|f| f.ms).collect();
        let newest = frames.last().map(|f| f.time).unwrap_or(0.0);
        let recent_ms: Vec<f64> = frames.iter().filter(|f| f.time >= newest - 3.0).map(|f| f.ms).collect();
        FrameStats {
            fps: average_fps(&ms),
            base_fps: base_fps(frames),
            one_percent_low: low_fps(&ms, 0.01),
            point_one_percent_low: low_fps(&ms, 0.001),
            spikes: spikes(&recent_ms),
            recent_ms,
        }
    }
}

actions!(overlay, [ToggleOverlay]);

pub const OVERLAY_WIDTH: f32 = 420.0;
pub const OVERLAY_HEIGHT: f32 = 56.0;

/// Where the overlay's frames come from: a live trace, or a test's capture.
pub type FrameFeed = Arc<Mutex<FrameWindow>>;

/// The game overlay strip: FPS, base FPS, lows, a frame-time graph.
pub struct OverlayView {
    pub feed: FrameFeed,
    /// Why there is no live data, if there is none.
    pub reason: Option<LiveError>,
    _tick: Task<()>,
}

impl OverlayView {
    pub fn new(feed: FrameFeed, reason: Option<LiveError>, cx: &mut Context<Self>) -> Self {
        let tick = cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor().timer(Duration::from_millis(500)).await;
                if this.update(cx, |_, cx| cx.notify()).is_err() {
                    break;
                }
            }
        });
        OverlayView { feed, reason, _tick: tick }
    }

    /// The current numbers, from whatever is in the feed.
    pub fn stats(&self) -> FrameStats {
        let window = self.feed.lock().unwrap_or_else(|p| p.into_inner());
        let frames: Vec<Frame> = window.frames.iter().copied().collect();
        FrameStats::compute(&frames)
    }
}

impl Render for OverlayView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let stats = self.stats();
        let empty = stats.recent_ms.is_empty();
        let cell = |name: &'static str, label: &'static str, value: String| {
            div().debug_selector(move || format!("overlay-{name}")).flex().gap_1().child(div().text_color(rgb(0x9a9a9a)).child(label)).child(value)
        };
        let reason = self.reason.clone();
        div()
            .debug_selector(|| "overlay".into())
            .size_full()
            .flex()
            .items_center()
            .gap_3()
            .px_3()
            .rounded(px(8.))
            .bg(rgba(0x000000aa))
            .text_color(rgb(0xffffff))
            .font_family("Consolas")
            .when(empty, |el| {
                el.child(cell("fps", "FPS", "--".into())).when_some(reason, |el, why| {
                    let text = match why {
                        LiveError::NeedsAdmin => "needs administrator".to_string(),
                        LiveError::Failed(text) => text,
                    };
                    el.child(div().text_xs().text_color(rgb(0x9a9a9a)).child(text))
                })
            })
            .when(!empty, |el| {
                el.child(cell("fps", "FPS", format!("{:.0}", stats.fps)))
                    .child(cell("base", "BASE", format!("{:.0}", stats.base_fps)))
                    .child(cell("low1", "1%", format!("{:.0}", stats.one_percent_low)))
                    .child(cell("low01", "0.1%", format!("{:.0}", stats.point_one_percent_low)))
                    .child(frame_graph(stats.recent_ms.clone(), stats.spikes.clone()))
            })
    }
}

/// The last 3 seconds of frame times; stutters drawn as red ticks.
fn frame_graph(ms: Vec<f64>, spikes: Vec<usize>) -> Div {
    div().debug_selector(|| "overlay-graph".into()).w(px(120.)).h(px(32.)).child(
        canvas(
            |_, _, _| {},
            move |bounds, _, window, _| {
                let points = graph_points(&ms, bounds);
                let mut line = PathBuilder::stroke(px(1.));
                for (i, p) in points.iter().enumerate() {
                    if i == 0 { line.move_to(*p) } else { line.line_to(*p) }
                }
                if let Ok(path) = line.build() {
                    window.paint_path(path, rgb(0x5ab0ff));
                }
                for i in &spikes {
                    if let Some(p) = points.get(*i) {
                        let mut tick = PathBuilder::stroke(px(2.));
                        tick.move_to(point(p.x, bounds.top()));
                        tick.line_to(point(p.x, bounds.bottom()));
                        if let Ok(path) = tick.build() {
                            window.paint_path(path, rgb(0xff5c5c));
                        }
                    }
                }
            },
        )
        .size_full(),
    )
}

/// Open the overlay strip at the top of the screen, or close it if it is open.
pub fn toggle_overlay_window(cx: &mut App, slot: &mut Option<WindowHandle<OverlayView>>, feed: FrameFeed, reason: Option<LiveError>) {
    if let Some(handle) = slot.take()
        && handle.update(cx, |_, window, _| window.remove_window()).is_ok()
    {
        return;
    }
    let screen = cx.primary_display().map(|d| d.bounds()).unwrap_or_default();
    let options = WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(overlay_bounds(screen, px(OVERLAY_WIDTH), px(OVERLAY_HEIGHT)))),
        titlebar: None,
        kind: WindowKind::PopUp,
        focus: false,
        is_movable: false,
        window_background: WindowBackgroundAppearance::Transparent,
        ..Default::default()
    };
    *slot = cx.open_window(options, |_, cx| cx.new(|cx| OverlayView::new(feed, reason, cx))).ok();
}

impl StatsBar {
    /// Open or close the game overlay (deferred, like the widget).
    pub fn toggle_overlay(&mut self, cx: &mut Context<Self>) {
        let mut slot = self.ui.overlay_window.take();
        let feed = self.ui.frame_feed.clone();
        let reason = self.ui.live_reason.clone();
        let bar = cx.entity();
        cx.defer(move |cx| {
            toggle_overlay_window(cx, &mut slot, feed, reason);
            bar.update(cx, |this, _| this.ui.overlay_window = slot);
        });
    }
}
