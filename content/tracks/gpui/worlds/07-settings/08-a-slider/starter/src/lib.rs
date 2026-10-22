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

/// The Appearance page's Gap slider: press or drag on the track to set the space between stats.
pub fn gap_slider(this: &SettingsWindow, cx: &mut Context<SettingsWindow>) -> impl IntoElement {
    let gap = cx.global::<Settings>().gap;
    let track = this.gap_track.clone();
    let share = gap / 24.0;
    div()
        .id("gap-slider")
        .debug_selector(|| "gap-slider".into())
        .w(px(240.))
        .h(px(20.))
        .relative()
        .child(canvas(move |bounds, _, _| track.set(bounds), |_, _, _, _| {}).size_full())
        .child(div().absolute().top(px(8.)).h(px(4.)).w_full().rounded_full().bg(rgb(0x555555)))
        .child(div().absolute().top(px(2.)).left(relative(share * 0.92)).size(px(16.)).rounded_full().bg(rgb(0x5ab0ff)))
        // TODO: on a left press, and on a move while the left button is held: this.set_gap_at(x, cx)
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
    fn dragging_slider_changes_gap(cx: &mut TestAppContext) {
        let (handle, mut vcx) = settings_window(cx);
        show_page(handle, &mut vcx, Page::Appearance);
        let track = vcx.debug_bounds("gap-slider").expect("the Gap slider is drawn");
        vcx.simulate_event(MouseDownEvent { position: point(track.left() + px(1.), track.center().y), modifiers: Modifiers::none(), button: MouseButton::Left, click_count: 1, first_mouse: false });
        assert_eq!(settings(&mut vcx).gap, 0.0, "pressing at the left edge sets 0");
        vcx.simulate_mouse_move(track.center(), MouseButton::Left, Modifiers::none());
        vcx.simulate_event(MouseUpEvent { position: track.center(), modifiers: Modifiers::none(), button: MouseButton::Left, click_count: 1 });
        assert_eq!(settings(&mut vcx).gap, 12.0, "dragging to the middle sets 12");
    }
}
