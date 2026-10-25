//! Kinetik Stats: the bar on the taskbar.

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
pub mod gpu;
pub mod net;
pub mod ping;
pub mod power;
pub mod real;
pub mod sensor;
pub mod thermal;
pub mod uptime;
pub mod view;
pub mod dock;
pub mod docker;
pub mod win;
pub mod window;

pub use bar::{SensorRead, StatsBar, clamp_interval, read_sensors, wanted_kinds};
pub use disk::DiskSensor;
pub use gpu::{GpuSensor, NvidiaGpu};
pub use net::NetSpeedSensor;
pub use ping::PingSensor;
pub use power::BatterySensor;
pub use real::{CpuSensor, RamSensor};
pub use sensor::{Missing, ScriptedSensor, Sensor, missing_text};
pub use thermal::ThermalSensor;
pub use view::{LabelMode, Look};
pub use dock::{Call, FakeWin, Rect, TRAY_GAP, Taskbar, WinApi, bar_visible, covers_monitor, dock_bar, taskbars_to_use, to_logical};
pub use docker::Docker;
pub use win::{RealWin, find_taskbar, hwnd_of};
pub use window::bar_window_options;

impl<A: WinApi> Docker<A> {
    /// Once a second: dock against the taskbar, raise the bar after a move, and show or hide it
    /// for an auto-hidden taskbar or a fullscreen window on its monitor.
    pub fn tick(&mut self) {
        let Some(taskbar) = self.api.taskbars().into_iter().find(|t| t.primary) else { return };
        let docked = dock_bar(&taskbar, self.bar_width, self.shift_left);
        if self.last != Some(taskbar) {
            self.last = Some(taskbar);
            if let Some(place) = docked {
                self.api.move_to(self.hwnd, place.left, place.top);
                self.api.set_topmost(self.hwnd);
            }
        }
        let visible = docked.is_some() && bar_visible(self.api.foreground(), taskbar.monitor);
        if visible != self.visible {
            self.visible = visible;
            self.api.show(self.hwnd, visible);
        }
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
#[allow(unused_imports, dead_code)]
mod tests {
    use super::*;
    use gpui::{TestAppContext, prelude::*};

    /// A 2560 x 1440 monitor with a 48 px taskbar at the bottom and a 520 px tray.
    fn taskbar() -> Taskbar {
        Taskbar {
            rect: Rect::new(0, 1392, 2560, 1440),
            tray: Rect::new(2040, 1392, 2560, 1440),
            monitor: Rect::new(0, 0, 2560, 1440),
            scale: 1.0,
            primary: true,
        }
    }

    fn docker(win: FakeWin) -> Docker<FakeWin> {
        Docker::new(win, 9, 300)
    }

    fn game_on(monitor: Rect) -> Option<(Rect, Rect)> {
        Some((monitor, monitor))
    }

    #[test]
    fn window_options_right() {
        let o = bar_window_options(to_logical(Rect::new(1500, 1392, 1800, 1440), 1.5));
        assert!(o.titlebar.is_none() && !o.focus && o.kind == gpui::WindowKind::PopUp);
        assert_eq!(o.window_background, gpui::WindowBackgroundAppearance::Transparent);
    }

    #[test]
    fn docks_left_of_tray() {
        let mut d = docker(FakeWin { taskbars: vec![taskbar()], ..Default::default() });
        d.tick();
        assert_eq!(d.api.calls.first(), Some(&Call::Move(9, 1732, 1392)));
    }

    #[test]
    fn stays_topmost() {
        let mut d = docker(FakeWin { taskbars: vec![taskbar()], ..Default::default() });
        d.tick();
        assert!(d.api.calls.contains(&Call::Topmost(9)), "the bar is raised after docking: {:?}", d.api.calls);
    }

    #[test]
    fn hides_for_fullscreen() {
        let mut d = docker(FakeWin { taskbars: vec![taskbar()], ..Default::default() });
        d.tick();
        d.api.foreground = game_on(taskbar().monitor);
        d.tick();
        assert_eq!(d.api.calls.last(), Some(&Call::Show(9, false)));
        d.api.foreground = None;
        d.tick();
        assert_eq!(d.api.calls.last(), Some(&Call::Show(9, true)));
        let shows = d.api.calls.iter().filter(|c| matches!(c, Call::Show(..))).count();
        assert_eq!(shows, 2, "show and hide only on a change");
    }

    #[test]
    fn redocks_after_move() {
        let mut d = docker(FakeWin { taskbars: vec![taskbar()], ..Default::default() });
        d.tick();
        d.tick();
        d.api.taskbars = vec![Taskbar { tray: Rect::new(1940, 1392, 2560, 1440), ..taskbar() }];
        d.tick();
        let moves: Vec<_> = d.api.calls.iter().filter(|c| matches!(c, Call::Move(..))).cloned().collect();
        assert_eq!(moves, vec![Call::Move(9, 1732, 1392), Call::Move(9, 1632, 1392)]);
    }

    #[test]
    fn hidden_taskbar_hides_bar() {
        let hidden = Taskbar { rect: Rect::new(0, 1438, 2560, 1486), tray: Rect::new(2040, 1438, 2560, 1486), ..taskbar() };
        let mut d = docker(FakeWin { taskbars: vec![hidden], ..Default::default() });
        d.tick();
        assert!(!d.visible);
        assert_eq!(d.api.calls, vec![Call::Show(9, false)]);
        d.api.taskbars = vec![taskbar()];
        d.tick();
        assert!(d.visible, "the taskbar slid up: show the bar again");
    }

    #[test]
    fn scales_with_dpi() {
        let b = to_logical(Rect::new(1500, 1392, 1800, 1440), 1.5);
        assert_eq!(b.size, gpui::size(gpui::px(200.), gpui::px(32.)));
    }
}
