//! The real Windows: taskbars, the window in front, topmost, move, show.

use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::Graphics::Gdi::{GetMonitorInfoW, MONITOR_DEFAULTTONEAREST, MONITORINFO, MonitorFromWindow};
use windows::Win32::UI::HiDpi::GetDpiForWindow;
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowExW, FindWindowW, GWL_EXSTYLE, GetForegroundWindow, GetWindowLongPtrW, GetWindowRect, HWND_BOTTOM,
    HWND_NOTOPMOST, HWND_TOPMOST, SetWindowLongPtrW, SW_HIDE, SW_SHOWNOACTIVATE,
    SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER, SetWindowPos, ShowWindow,
};
use windows::core::w;

use crate::dock::{Layer, Rect, Taskbar, WinApi};

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

/// Secondary taskbars have no tray: the bar goes against their right end instead.
fn secondary_taskbars() -> Vec<Taskbar> {
    let mut found = Vec::new();
    let mut after: Option<HWND> = None;
    // SAFETY: plain window lookups; every handle is checked before use.
    unsafe {
        while let Ok(hwnd) = FindWindowExW(None, after, w!("Shell_SecondaryTrayWnd"), None) {
            if let (Some(rect), Some(monitor)) = (window_rect(hwnd), monitor_rect(hwnd)) {
                let tray = Rect::new(rect.right, rect.top, rect.right, rect.bottom);
                let scale = GetDpiForWindow(hwnd) as f32 / 96.0;
                found.push(Taskbar { rect, tray, monitor, scale, primary: false });
            }
            after = Some(hwnd);
        }
    }
    found
}

/// The real Windows, through the `windows` crate.
#[derive(Debug, Default)]
pub struct RealWin;

impl WinApi for RealWin {
    fn taskbars(&self) -> Vec<Taskbar> {
        find_taskbar().into_iter().chain(secondary_taskbars()).collect()
    }

    fn foreground(&self) -> Option<(Rect, Rect)> {
        // SAFETY: GetForegroundWindow has no inputs; the handle is checked.
        let hwnd = unsafe { GetForegroundWindow() };
        if hwnd.is_invalid() {
            return None;
        }
        Some((window_rect(hwnd)?, monitor_rect(hwnd)?))
    }

    fn set_topmost(&mut self, hwnd: isize) {
        let flags = SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE;
        // SAFETY: SetWindowPos only reads the handle; a stale handle makes it fail, not crash.
        let _ = unsafe { SetWindowPos(HWND(hwnd as *mut _), Some(HWND_TOPMOST), 0, 0, 0, 0, flags) };
    }

    fn move_to(&mut self, hwnd: isize, x: i32, y: i32) {
        let flags = SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE;
        // SAFETY: as above. Never resize through Win32: GPUI would draw at the wrong size.
        let _ = unsafe { SetWindowPos(HWND(hwnd as *mut _), None, x, y, 0, 0, flags) };
    }

    fn show(&mut self, hwnd: isize, visible: bool) {
        let how = if visible { SW_SHOWNOACTIVATE } else { SW_HIDE };
        // SAFETY: ShowWindow only reads the handle.
        let _ = unsafe { ShowWindow(HWND(hwnd as *mut _), how) };
    }

    fn set_layer(&mut self, hwnd: isize, layer: Layer) {
        let after = match layer {
            Layer::Bottom => HWND_BOTTOM,
            Layer::Normal => HWND_NOTOPMOST,
            Layer::Top => HWND_TOPMOST,
        };
        let flags = SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE;
        // SAFETY: SetWindowPos only reads the handle.
        let _ = unsafe { SetWindowPos(HWND(hwnd as *mut _), Some(after), 0, 0, 0, 0, flags) };
    }

    fn set_click_through(&mut self, hwnd: isize, on: bool) {
        let hwnd = HWND(hwnd as *mut _);
        // SAFETY: reading and writing a window's extended style only touches that window.
        unsafe {
            let style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE) as u32;
            SetWindowLongPtrW(hwnd, GWL_EXSTYLE, crate::click_through_style(style, on) as isize);
        }
    }
}

/// The Win32 handle of a GPUI window.
pub fn hwnd_of(window: &gpui::Window) -> Option<isize> {
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};
    // `Window` has its own `window_handle()`, so name the raw-window-handle trait's one.
    match HasWindowHandle::window_handle(window).ok()?.as_raw() {
        RawWindowHandle::Win32(h) => Some(h.hwnd.get()),
        _ => None,
    }
}
