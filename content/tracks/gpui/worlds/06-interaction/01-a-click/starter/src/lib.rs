//! Kinetik Stats: a bar you can click, hover and right-click.

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
pub mod gpu;
pub mod net;
pub mod ping;
pub mod power;
pub mod real;
pub mod sensor;
pub mod thermal;
pub mod ui;
pub mod uptime;
pub mod view;
pub mod win;
pub mod window;

pub use bar::{SensorRead, StatsBar, clamp_interval, read_sensors, wanted_kinds};
pub use disk::DiskSensor;
pub use dock::{Call, FakeWin, Rect, Taskbar, WinApi, bar_visible, covers_monitor, dock_bar, taskbars_to_use, to_logical};
pub use docker::Docker;
pub use gpu::{GpuSensor, NvidiaGpu};
pub use net::NetSpeedSensor;
pub use ping::PingSensor;
pub use power::BatterySensor;
pub use real::{CpuSensor, RamSensor};
pub use sensor::{Missing, ScriptedSensor, Sensor, missing_text};
pub use thermal::ThermalSensor;
pub use ui::{BarUi, FakeLauncher, LeftClick, Launcher, ProcessUse, RealLauncher};
pub use view::{LabelMode, Look};
pub use win::{RealWin, find_taskbar, hwnd_of};
pub use window::bar_window_options;

use gpui::{Context, Div, SharedString, Stateful, prelude::*};

/// Everything a pill does when you touch it.
pub fn interactive_pill(pill: Div, kind: StatKind, bar: &StatsBar, cx: &mut Context<StatsBar>) -> Stateful<Div> {
    let _ = bar;
    pill
        // TODO: .id(SharedString::from(format!("pill-{kind:?}"))) so the pill can take clicks
        .id("pill")
        // TODO: .on_click that sets this.ui.clicked to Some(kind) and notifies
}

/// Things that belong to the whole bar, not one pill. Nothing yet.
pub fn bar_layer(bar: Div, _this: &StatsBar, _cx: &mut Context<StatsBar>) -> Div {
    bar
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
    use gpui::{Modifiers, MouseButton, MouseDownEvent, MouseUpEvent, TestAppContext, VisualTestContext, point, prelude::*, px};
    use std::time::Duration;

    /// A bar with only a scripted CPU sensor, focused, in a test window; three reads in.
    fn open(cx: &mut TestAppContext) -> (gpui::Entity<StatsBar>, &mut VisualTestContext) {
        let (bar, cx) = cx.add_window_view(|window, cx| {
            let sensors: Vec<Box<dyn Sensor>> = vec![Box::new(ScriptedSensor::new(StatKind::CpuUsage, vec![10.0, 20.0, 30.0]))];
            let mut bar = StatsBar::new(sensors);
            bar.ui.launcher = Box::new(FakeLauncher::default());
            bar.ui.top_processes = || vec![("game.exe".into(), 4 << 30), ("browser.exe".into(), 2 << 30)];
            bar.focus_in(window, cx);
            bar.start(cx);
            bar
        });
        for _ in 0..3 {
            cx.executor().advance_clock(Duration::from_secs(1));
            cx.run_until_parked();
        }
        (bar, cx)
    }

    fn centre_of(cx: &mut VisualTestContext, name: &'static str) -> gpui::Point<gpui::Pixels> {
        cx.debug_bounds(name).unwrap_or_else(|| panic!("nothing named {name} was drawn")).center()
    }

    fn hover(cx: &mut VisualTestContext, name: &'static str) {
        let at = centre_of(cx, name);
        cx.simulate_mouse_move(at, None, Modifiers::none());
        cx.executor().advance_clock(Duration::from_millis(600));
        cx.run_until_parked();
    }

    fn move_away(cx: &mut VisualTestContext) {
        cx.simulate_mouse_move(point(px(1500.), px(900.)), None, Modifiers::none());
        cx.executor().advance_clock(Duration::from_millis(600));
        cx.run_until_parked();
    }

    fn right_click(cx: &mut VisualTestContext, name: &'static str) {
        let at = centre_of(cx, name);
        cx.simulate_event(MouseDownEvent { position: at, modifiers: Modifiers::none(), button: MouseButton::Right, click_count: 1, first_mouse: false });
        cx.simulate_event(MouseUpEvent { position: at, modifiers: Modifiers::none(), button: MouseButton::Right, click_count: 1 });
        cx.run_until_parked();
    }

    #[gpui::test]
    fn clicking_cpu_pill_is_seen(cx: &mut TestAppContext) {
        let (bar, cx) = open(cx);
        let at = centre_of(cx, "pill-CpuUsage");
        cx.simulate_click(at, Modifiers::none());
        assert_eq!(bar.read_with(cx, |bar, _| bar.ui.clicked), Some(StatKind::CpuUsage));
    }
}
