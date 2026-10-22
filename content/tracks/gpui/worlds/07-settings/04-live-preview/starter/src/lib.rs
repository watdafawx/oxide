//! Kinetik Stats: the Settings window.

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
pub mod win;
pub mod window;

pub use bar::{SensorRead, StatsBar, clamp_interval, read_sensors, wanted_kinds};
pub use disk::DiskSensor;
pub use dock::{Call, FakeWin, Rect, Taskbar, WinApi, bar_visible, covers_monitor, dock_bar, taskbars_to_use, to_logical};
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
pub use win::{RealWin, find_taskbar, hwnd_of};
pub use window::bar_window_options;

#[allow(unused_imports)]
use gpui::{App, Bounds, Context, Div, Global, IntoElement, KeyBinding, KeyDownEvent, MouseButton, MouseDownEvent, MouseMoveEvent, Pixels, RenderOnce, SharedString, Stateful, Subscription, TitlebarOptions, UpdateGlobal, Window, WindowBounds, WindowHandle, WindowOptions, canvas, div, prelude::*, px, relative, rgb, size, uniform_list};
#[allow(unused_imports)]
use std::{collections::BTreeMap, fs, io, ops::Range, path::{Path, PathBuf}};
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

impl StatsBar {
    /// Redraw with the new settings whenever they change: the live preview.
    pub fn watch_settings(&mut self, cx: &mut Context<Self>) {
        let _ = cx;
        // TODO: observe the Settings global: apply the new settings to the bar, then notify;
        // keep the subscription in self.settings_watch
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
    use gpui::{Modifiers, MouseButton, MouseDownEvent, MouseUpEvent, TestAppContext, UpdateGlobal, VisualTestContext, WindowHandle, point, prelude::*, px};
    use std::time::Duration;

    fn with_settings(cx: &mut TestAppContext) {
        cx.update(|cx| {
            cx.set_global(Settings::default());
            
        });
    }

    /// Opens the Settings window and gives a context to drive it.
    fn settings_window(cx: &mut TestAppContext) -> (WindowHandle<SettingsWindow>, VisualTestContext) {
        with_settings(cx);
        let mut slot = None;
        cx.update(|cx| open_settings_window(cx, &mut slot));
        let handle = slot.expect("the Settings window opened");
        let mut vcx = VisualTestContext::from_window(handle.into(), cx);
        vcx.run_until_parked();
        (handle, vcx)
    }

    fn settings(cx: &mut VisualTestContext) -> Settings {
        cx.update(|_, cx| cx.global::<Settings>().clone())
    }

    fn click(cx: &mut VisualTestContext, name: &'static str) {
        let at = cx.debug_bounds(name).unwrap_or_else(|| panic!("nothing named {name} was drawn")).center();
        cx.simulate_click(at, Modifiers::none());
        cx.run_until_parked();
    }

    fn show_page(handle: WindowHandle<SettingsWindow>, cx: &mut VisualTestContext, page: Page) {
        handle.update(cx, |w, _, cx| { w.page = page; cx.notify(); }).unwrap();
        cx.run_until_parked();
    }

    #[gpui::test]
    fn changing_font_size_redraws_bar(cx: &mut TestAppContext) {
        with_settings(cx);
        let bar = cx.new(|cx| {
            let mut bar = StatsBar::from_settings(Vec::new(), cx);
            bar.watch_settings(cx);
            bar
        });
        let notified = std::rc::Rc::new(std::cell::Cell::new(0));
        let seen = notified.clone();
        cx.update(|cx| cx.observe(&bar, move |_, _| seen.set(seen.get() + 1)).detach());
        cx.update(|cx| Settings::update_global(cx, |s, _| s.font_size = 20.0));
        cx.run_until_parked();
        assert_eq!(bar.read_with(cx, |bar, _| bar.look.font_size), 20.0);
        assert!(notified.get() >= 1, "the bar should redraw");
    }
}
