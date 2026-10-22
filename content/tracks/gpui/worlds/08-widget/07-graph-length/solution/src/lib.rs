//! Kinetik Stats: the desktop widget.

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
pub mod interact;
pub mod net;
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
pub use gpu::{GpuSensor, NvidiaGpu};
pub use interact::*;
pub use net::NetSpeedSensor;
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
use gpui::{App, Context, Div, Entity, IntoElement, MouseButton, PathBuilder, Render, Stateful, Subscription, Window, WindowBackgroundAppearance, WindowBounds, WindowHandle, WindowKind, WindowOptions, canvas, div, prelude::*, px, relative, rgb, rgba};

/// How many readings a graph of `secs` seconds holds at one reading per `interval_ms`.
pub fn history_points(secs: u32, interval_ms: u64) -> usize {
    (secs as u64 * 1000 / interval_ms.max(1)) as usize
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
    use gpui::{Modifiers, MouseButton, MouseDownEvent, TestAppContext, UpdateGlobal, VisualTestContext, WindowHandle, prelude::*};
    use std::time::Duration;

    /// A bar reading scripted CPU (50 %, then 60 %) and RAM, after two reads.
    fn ticking_bar(cx: &mut TestAppContext) -> Entity<StatsBar> {
        cx.update(|cx| cx.set_global(Settings::default()));
        let bar = cx.new(|cx| {
            let sensors: Vec<Box<dyn Sensor>> = vec![
                Box::new(ScriptedSensor::new(StatKind::CpuUsage, vec![50.0, 50.0, 50.0])),
                Box::new(ScriptedSensor::new(StatKind::CpuTemp, vec![60.0, 61.0, 62.0])),
                Box::new(ScriptedSensor::new(StatKind::GpuUsage, vec![30.0, 30.0, 30.0])),
            ];
            let mut bar = StatsBar::from_settings(sensors, cx);
            bar.watch_settings(cx);
            bar.start(cx);
            bar
        });
        for _ in 0..2 {
            cx.executor().advance_clock(Duration::from_secs(1));
            cx.run_until_parked();
        }
        bar
    }

    /// Opens the widget for `bar`, with 16 fake cores and window dragging switched off.
    fn widget(cx: &mut TestAppContext, bar: Entity<StatsBar>) -> (WindowHandle<WidgetView>, VisualTestContext) {
        let mut slot = None;
        cx.update(|cx| toggle_widget_window(cx, &mut slot, bar.clone()));
        let handle = slot.expect("the widget opened");
        let mut vcx = VisualTestContext::from_window(handle.into(), cx);
        handle
            .update(&mut vcx, |w, _, cx| {
                w.core_source = || vec![40.0; 16];
                w.cores = (w.core_source)();
                w.start_move = |_| {};
                cx.notify();
            })
            .unwrap();
        vcx.run_until_parked();
        (handle, vcx)
    }

    fn bounds(cx: &mut VisualTestContext, name: &'static str) -> gpui::Bounds<gpui::Pixels> {
        cx.debug_bounds(name).unwrap_or_else(|| panic!("nothing named {name} was drawn"))
    }

    fn set_widget(cx: &mut VisualTestContext, change: impl FnOnce(&mut WidgetSettings)) {
        cx.update(|_, cx| Settings::update_global(cx, |s, _| change(&mut s.widget)));
        cx.run_until_parked();
    }

    fn work() -> Rect {
        Rect::new(0, 0, 2560, 1392)
    }

    #[test]
    fn five_minutes_keeps_300_points() {
        assert_eq!(history_points(300, 1000), 300);
        assert_eq!(history_points(30, 500), 60);
        assert_eq!(history_points(60, 0), 60_000, "a 0 interval does not panic");
    }
}
