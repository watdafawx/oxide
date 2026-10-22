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
    /// Copy what the bar shows from `settings`.
    pub fn apply_settings(&mut self, settings: &Settings) {
        let _ = settings;
        todo!("copy the look, order, colours and left-click action, then rebuild the items")
    }
}

/// Saved setups by name, like Kinetik's Gaming and Work profiles.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Profiles {
    pub saved: BTreeMap<String, Settings>,
}

impl Profiles {
    /// Keep a copy of the current settings under `name`.
    pub fn save_current(&mut self, name: &str, cx: &App) {
        let _ = (name, cx);
        todo!("store a clone of the current Settings under name")
    }

    /// Make a saved profile the current settings. Unknown names change nothing.
    pub fn switch_to(&self, name: &str, cx: &mut App) -> bool {
        let _ = (name, cx);
        todo!("set_global the saved settings for name; false for an unknown name")
    }
}

/// Save the settings to `path` every time they change.
pub fn persist_settings(path: PathBuf, cx: &mut App) -> Subscription {
    let _ = path;
    todo!("observe the Settings global and save it to path on every change")
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

    fn temp_file(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("kinetik-07-{name}-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        dir.join("settings.json")
    }

    fn watching_bar(cx: &mut TestAppContext) -> gpui::Entity<StatsBar> {
        with_settings(cx);
        cx.new(|cx| {
            let mut bar = StatsBar::from_settings(Vec::new(), cx);
            bar.watch_settings(cx);
            bar
        })
    }

    #[gpui::test]
    fn one_settings_window(cx: &mut TestAppContext) {
        with_settings(cx);
        let (bar, vcx) = cx.add_window_view(|window, cx| {
            let mut bar = StatsBar::from_settings(Vec::new(), cx);
            bar.focus_in(window, cx);
            bar
        });
        vcx.dispatch_action(OpenSettings);
        vcx.dispatch_action(OpenSettings);
        assert_eq!(vcx.update(|_, cx| cx.windows().len()), 2, "the bar and one Settings window");
    }

    #[gpui::test]
    fn toggle_hides_stat_on_bar(cx: &mut TestAppContext) {
        let bar = watching_bar(cx);
        assert!(bar.read_with(cx, |bar, _| bar.text_of(StatKind::CpuUsage)).is_some());
        cx.update(|cx| Settings::update_global(cx, |s, _| set_stat_on(&mut s.order, StatKind::CpuUsage, false)));
        cx.run_until_parked();
        assert_eq!(bar.read_with(cx, |bar, _| bar.text_of(StatKind::CpuUsage)), None);
    }

    #[gpui::test]
    fn slider_changes_bar_live(cx: &mut TestAppContext) {
        let bar = watching_bar(cx);
        cx.update(|cx| Settings::update_global(cx, |s, _| s.gap = 16.0));
        cx.run_until_parked();
        assert_eq!(bar.read_with(cx, |bar, _| bar.look.gap), 16.0);
    }

    #[gpui::test]
    fn arrange_drag_reorders_bar(cx: &mut TestAppContext) {
        let bar = watching_bar(cx);
        cx.update(|cx| Settings::update_global(cx, |s, _| move_slot(&mut s.order, 0, 3)));
        cx.run_until_parked();
        assert_eq!(bar.read_with(cx, |bar, _| bar.order[3]), Slot::Stat(StatKind::CpuUsage));
    }

    #[gpui::test]
    fn alt_up_reorders(cx: &mut TestAppContext) {
        let bar = watching_bar(cx);
        cx.update(bind_settings_keys);
        let mut slot = None;
        cx.update(|cx| open_settings_window(cx, &mut slot));
        let handle = slot.unwrap();
        let mut vcx = VisualTestContext::from_window(handle.into(), cx);
        show_page(handle, &mut vcx, Page::Arrange);
        click(&mut vcx, "row-1");
        vcx.simulate_keystrokes("alt-up");
        vcx.run_until_parked();
        assert_eq!(bar.read_with(&vcx, |bar, _| bar.order[0]), Slot::Stat(StatKind::CpuTemp), "the bar follows the list");
    }

    #[gpui::test]
    fn settings_survive_restart(cx: &mut TestAppContext) {
        with_settings(cx);
        let path = temp_file("persist");
        let watch = cx.update(|cx| persist_settings(path.clone(), cx));
        cx.update(|cx| Settings::update_global(cx, |s, _| s.font_size = 18.0));
        cx.run_until_parked();
        assert_eq!(load(&path).font_size, 18.0, "the change was saved");
        drop(watch);
    }

    #[gpui::test]
    fn switch_to_gaming_profile(cx: &mut TestAppContext) {
        with_settings(cx);
        let mut profiles = Profiles::default();
        cx.update(|cx| profiles.save_current("Work", cx));
        cx.update(|cx| Settings::update_global(cx, |s, _| { s.two_lines = true; s.font_size = 11.0; }));
        cx.update(|cx| profiles.save_current("Gaming", cx));
        assert!(cx.update(|cx| profiles.switch_to("Work", cx)));
        assert!(!cx.update(|cx| cx.global::<Settings>().two_lines));
        assert!(cx.update(|cx| profiles.switch_to("Gaming", cx)));
        assert_eq!(cx.update(|cx| cx.global::<Settings>().font_size), 11.0);
        assert!(!cx.update(|cx| profiles.switch_to("Streaming", cx)), "unknown profile");
    }

    #[test]
    fn bad_file_does_not_crash() {
        let path = temp_file("boss-bad");
        std::fs::write(&path, "{ broken").unwrap();
        assert_eq!(load(&path), Settings::default());
    }
}
