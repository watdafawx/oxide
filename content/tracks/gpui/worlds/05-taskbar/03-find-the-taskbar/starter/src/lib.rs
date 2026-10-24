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
pub use dock::{Rect, Taskbar};
pub use window::bar_window_options;

use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::Graphics::Gdi::{GetMonitorInfoW, MONITOR_DEFAULTTONEAREST, MONITORINFO, MonitorFromWindow};
use windows::Win32::UI::HiDpi::GetDpiForWindow;
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowExW, FindWindowW, GetWindowRect,
};
use windows::core::w;

/// The primary taskbar, read from Windows: Shell_TrayWnd and its TrayNotifyWnd child.
pub fn find_taskbar() -> Option<Taskbar> {
    // SAFETY: plain window lookups; every handle is checked before use.
    unsafe {
        // TODO: look up the window of class "Shell_TrayWnd", then its child of class "TrayNotifyWnd";
        // give up with None when either is missing
        let taskbar: HWND = todo!("look up Shell_TrayWnd");
        let tray: HWND = todo!("look up its TrayNotifyWnd child");
        Some(Taskbar {
            rect: window_rect(taskbar)?,
            tray: window_rect(tray)?,
            monitor: monitor_rect(taskbar)?,
            scale: GetDpiForWindow(taskbar) as f32 / 96.0,
            primary: true,
        })
    }
}

fn window_rect(hwnd: HWND) -> Option<Rect> {
    let mut r = RECT::default();
    // SAFETY: `r` is a valid RECT to write into.
    unsafe { GetWindowRect(hwnd, &mut r) }.ok()?;
    Some(Rect::new(r.left, r.top, r.right, r.bottom))
}

fn monitor_rect(hwnd: HWND) -> Option<Rect> {
    let mut info = MONITORINFO { cbSize: std::mem::size_of::<MONITORINFO>() as u32, ..Default::default() };
    // SAFETY: `info` has its size set, as GetMonitorInfoW requires.
    unsafe { GetMonitorInfoW(MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST), &mut info) }.ok().ok()?;
    let r = info.rcMonitor;
    Some(Rect::new(r.left, r.top, r.right, r.bottom))
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
mod tests {
    use super::*;

    #[test]
    fn taskbar_found_on_this_pc() {
        let t = find_taskbar().expect("Windows always has a Shell_TrayWnd");
        assert!(t.rect.width() > 0 && t.rect.height() > 0, "taskbar {:?}", t.rect);
        assert!(t.tray.left >= t.rect.left && t.tray.right <= t.rect.right, "the tray sits inside the taskbar");
        assert!(t.scale >= 1.0);
    }
}
