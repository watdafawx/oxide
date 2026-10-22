//! Kinetik Stats: the game overlay.

pub mod demo;
pub mod format;
pub mod history;
pub mod limits;
pub mod order;
pub mod pills;
pub mod reading;
pub mod stat;

pub use format::{format_bytes, format_percent, format_speed};
pub use history::{History, Summary};
pub use limits::{Limit, Limits, WarnWhen, is_warning};
pub use order::{DividerStyle, Slot, default_order, move_up};
pub use pills::{BarItem, Column, Pill, Snapshot, bar_items, hover_summary, two_line_columns};
pub use reading::Reading;
pub use stat::{StatKind, Unit};
pub mod bar;
pub mod disk;
pub mod dock;
pub mod docker;
pub mod frames;
pub mod gpu;
pub mod interact;
pub mod live;
pub mod net;
pub mod overlay;
pub mod ping;
pub mod power;
pub mod real;
pub mod sensor;
pub mod settings;
pub mod settings_window;
pub mod thermal;
pub mod ui;
pub mod uptime;
pub mod view;
pub mod widget;
pub mod win;
pub mod window;

pub use bar::{SensorRead, StatsBar, clamp_interval, read_sensors, wanted_kinds};
pub use disk::DiskSensor;
pub use dock::{Call, FakeWin, Layer, Rect, Taskbar, WinApi, bar_visible, covers_monitor, dock_bar, taskbars_to_use, to_logical};
pub use docker::Docker;
pub use frames::*;
pub use gpu::{GpuSensor, NvidiaGpu};
pub use interact::*;
pub use live::*;
pub use net::NetSpeedSensor;
#[allow(unused_imports)]
pub use overlay::*;
pub use ping::PingSensor;
pub use power::BatterySensor;
pub use real::{CpuSensor, RamSensor};
pub use sensor::{Missing, ScriptedSensor, Sensor, missing_text};
pub use settings::*;
pub use settings_window::*;
pub use thermal::ThermalSensor;
pub use ui::{BarUi, FakeLauncher, LeftClick, Launcher, ProcessUse, RealLauncher};
pub use view::{LabelMode, Look};
pub use widget::*;
pub use win::{RealWin, find_taskbar, hwnd_of};
pub use window::bar_window_options;

#[allow(unused_imports)]
use std::collections::VecDeque;
#[allow(unused_imports)]
use global_hotkey::{GlobalHotKeyEvent, HotKeyState, hotkey::{Code, HotKey, Modifiers}};
#[allow(unused_imports)]
use gpui::{Bounds, Pixels, point, px, size};
#[allow(unused_imports)]
use ferrisetw::{native::EvntraceNativeError, trace::TraceError};

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

/// The overlay's brain: auto-show, session summaries and benchmark rows, with no windows in it.
#[derive(Debug, Default)]
pub struct OverlayController {
    pub auto: AutoShow,
    pub sessions: Vec<String>,
    pub benchmark: Benchmark,
    started: Option<f64>,
    peak_gpu_temp: Option<f64>,
}

impl OverlayController {
    /// One check, once a second at `now` seconds. Gives back what the overlay window should do.
    pub fn tick(&mut self, now: f64, front: Front, game_exe: &str, running: impl Fn(u32) -> bool, stats: &FrameStats, gpu_temp: Option<f64>) -> Option<OverlayStep> {
        if self.auto.game.is_some() {
            self.peak_gpu_temp = match (self.peak_gpu_temp, gpu_temp) {
                (Some(a), Some(b)) => Some(a.max(b)),
                (a, b) => a.or(b),
            };
        }
        let step = self.auto.step(front, running);
        match step {
            Some(OverlayStep::Show) if self.started.is_none() => self.started = Some(now),
            Some(OverlayStep::Close) => {
                let started = self.started.take().unwrap_or(now);
                let summary = SessionSummary {
                    game: game_exe.to_string(),
                    seconds: (now - started).max(0.0) as u64,
                    average_fps: stats.fps,
                    one_percent_low: stats.one_percent_low,
                    peak_gpu_temp: self.peak_gpu_temp.take(),
                };
                self.sessions.push(summary.csv_line());
            }
            _ => {}
        }
        step
    }
}

/// Every sensor this PC has so far; the rest of the bar is scripted.
pub fn live_sensors() -> Vec<Box<dyn Sensor>> {
    let mut sensors: Vec<Box<dyn Sensor>> = vec![
        Box::new(CpuSensor::new()),
        Box::new(RamSensor::new()),
        Box::new(NetSpeedSensor::download()),
        Box::new(NetSpeedSensor::upload()),
        Box::new(DiskSensor::new()),
        Box::new(GpuSensor::usage()),
        Box::new(GpuSensor::temperature()),
        Box::new(ThermalSensor::new()),
        Box::new(BatterySensor),
        Box::new(PingSensor::new()),
    ];
    let real: Vec<StatKind> = sensors.iter().map(|s| s.kind()).collect();
    for kind in [StatKind::CpuUsage, StatKind::CpuTemp, StatKind::GpuUsage, StatKind::GpuTemp, StatKind::RamUsed, StatKind::NetDown, StatKind::NetUp, StatKind::Ping] {
        if !real.contains(&kind) {
            let script = (0..300).map(|i| 20.0 + (i % 7) as f64 * 5.0).collect();
            sensors.push(Box::new(ScriptedSensor::new(kind, script)));
        }
    }
    sensors
}

#[cfg(test)]
#[allow(unused_imports, dead_code, unused_variables, unused_mut)]
mod tests {
    use super::*;
    use gpui::{TestAppContext, VisualTestContext, prelude::*};

    /// The capture in fixtures/: 1200 frames of a game with frame generation, five stutters.
    fn capture() -> Vec<Frame> {
        parse_capture(include_str!("../fixtures/capture.csv"))
    }

    fn capture_ms() -> Vec<f64> {
        capture().iter().map(|f| f.ms).collect()
    }

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 0.01
    }

    fn controller_after_game(seconds: f64) -> (OverlayController, Option<OverlayStep>) {
        let stats = FrameStats::compute(&capture());
        let mut c = OverlayController::default();
        c.tick(0.0, Front::Game(7), "Cyberpunk2077.exe", |_| true, &stats, Some(70.0));
        c.tick(seconds / 2.0, Front::Game(7), "Cyberpunk2077.exe", |_| true, &stats, Some(74.0));
        let step = c.tick(seconds, Front::Other, "Cyberpunk2077.exe", |_| false, &stats, Some(60.0));
        (c, step)
    }

    #[test]
    fn fps_from_capture() {
        let s = FrameStats::compute(&capture());
        assert!(close(s.fps, 118.06) && close(s.base_fps, 59.03));
    }

    #[test]
    fn lows_from_capture() {
        let s = FrameStats::compute(&capture());
        assert!(close(s.one_percent_low, 64.31) && close(s.point_one_percent_low, 22.22));
    }

    #[test]
    fn graph_marks_spikes() {
        let s = FrameStats::compute(&capture());
        assert_eq!(s.recent_ms.len(), 356, "the last 3 seconds");
        assert_eq!(s.spikes, vec![156], "the stutter at frame 1000");
    }

    #[gpui::test]
    fn hotkey_toggles(cx: &mut TestAppContext) {
        cx.update(|cx| cx.set_global(Settings::default()));
        let (bar, vcx) = cx.add_window_view(|window, cx| {
            let mut bar = StatsBar::from_settings(Vec::new(), cx);
            let mut window_frames = FrameWindow::default();
            for frame in capture() {
                window_frames.push(frame);
            }
            bar.ui.frame_feed = std::sync::Arc::new(std::sync::Mutex::new(window_frames));
            bar.focus_in(window, cx);
            bar
        });
        vcx.dispatch_action(ToggleOverlay);
        vcx.run_until_parked();
        assert_eq!(vcx.update(|_, cx| cx.windows().len()), 2, "the bar and the overlay");
    }

    #[test]
    fn shows_for_game() {
        let stats = FrameStats::compute(&capture());
        let mut c = OverlayController::default();
        assert_eq!(c.tick(0.0, Front::Game(7), "x.exe", |_| true, &stats, None), Some(OverlayStep::Show));
        assert_eq!(c.tick(1.0, Front::Other, "x.exe", |_| true, &stats, None), Some(OverlayStep::Hide));
    }

    #[test]
    fn closes_with_game() {
        let (_, step) = controller_after_game(2550.0);
        assert_eq!(step, Some(OverlayStep::Close));
    }

    #[test]
    fn summary_saved() {
        let (c, _) = controller_after_game(2550.0);
        assert_eq!(c.sessions, vec!["Cyberpunk2077.exe,42.5,118.1,64.3,74".to_string()], "peak GPU temperature while playing");
    }

    #[test]
    fn benchmark_rows() {
        let mut c = OverlayController::default();
        c.benchmark.toggle();
        c.benchmark.tick(&Snapshot::new(), Some(60.0));
        assert_eq!(c.benchmark.rows, vec![BENCHMARK_HEADER.to_string(), "0,,,,60.0".to_string()]);
    }

    #[test]
    fn live_or_needs_admin() {
        match start_present_trace(|_, _| {}) {
            Ok(trace) => drop(trace),
            Err(why) => assert_eq!(why, LiveError::NeedsAdmin),
        }
    }
}
