//! Finding the taskbar.

use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::Graphics::Gdi::{GetMonitorInfoW, MONITOR_DEFAULTTONEAREST, MONITORINFO, MonitorFromWindow};
use windows::Win32::UI::HiDpi::GetDpiForWindow;
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowExW, FindWindowW, GetWindowRect,
};
use windows::core::w;

use crate::dock::{Rect, Taskbar};

/// The primary taskbar, read from Windows: Shell_TrayWnd and its TrayNotifyWnd child.
pub fn find_taskbar() -> Option<Taskbar> {
    // SAFETY: plain window lookups; every handle is checked before use.
    unsafe {
        let taskbar = FindWindowW(w!("Shell_TrayWnd"), None).ok()?;
        let tray = FindWindowExW(Some(taskbar), None, w!("TrayNotifyWnd"), None).ok()?;
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

