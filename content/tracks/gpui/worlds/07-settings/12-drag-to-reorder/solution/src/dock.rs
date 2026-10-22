//! Where the bar goes.

use gpui::{Bounds, Pixels, point, px, size};

/// A rectangle in physical screen pixels, the way Windows reports it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Rect {
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Rect { left, top, right, bottom }
    }

    pub fn width(&self) -> i32 {
        self.right - self.left
    }

    pub fn height(&self) -> i32 {
        self.bottom - self.top
    }

    /// The part of `self` inside `other`, if any.
    pub fn intersect(&self, other: &Rect) -> Option<Rect> {
        let r = Rect::new(
            self.left.max(other.left),
            self.top.max(other.top),
            self.right.min(other.right),
            self.bottom.min(other.bottom),
        );
        (r.width() > 0 && r.height() > 0).then_some(r)
    }
}

/// One taskbar: where it is, where its tray is, which monitor it sits on, and that monitor's scale.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Taskbar {
    pub rect: Rect,
    pub tray: Rect,
    pub monitor: Rect,
    /// 1.0 at 100 % scaling, 1.5 at 150 %.
    pub scale: f32,
    pub primary: bool,
}

/// The space between the bar and the tray, in physical pixels at 100 %.
pub const TRAY_GAP: i32 = 8;

/// Where the bar goes: left of the tray, as tall as the taskbar, moved left by `shift_left`.
/// None when the taskbar is hidden (auto-hide) and shows less than 4 pixels on its monitor.
pub fn dock_bar(taskbar: &Taskbar, bar_width: i32, shift_left: i32) -> Option<Rect> {
    let visible = taskbar.rect.intersect(&taskbar.monitor)?;
    if visible.height() < 4 {
        return None;
    }
    let right = taskbar.tray.left - TRAY_GAP - shift_left;
    Some(Rect::new(right - bar_width, visible.top, right, visible.bottom))
}

/// Windows thinks in physical pixels; GPUI windows are placed in logical ones.
/// At 150 % scaling, 300 physical pixels are 200 logical.
pub fn to_logical(rect: Rect, scale: f32) -> Bounds<Pixels> {
    Bounds::new(
        point(px(rect.left as f32 / scale), px(rect.top as f32 / scale)),
        size(px(rect.width() as f32 / scale), px(rect.height() as f32 / scale)),
    )
}

/// The few Windows calls the docked bar needs. Tests use `FakeWin`; the app uses `RealWin`.
pub trait WinApi {
    /// Every taskbar, the primary one first.
    fn taskbars(&self) -> Vec<Taskbar>;
    /// The window in front and the monitor it is on, if any.
    fn foreground(&self) -> Option<(Rect, Rect)>;
    /// Put a window above everything that is not also topmost.
    fn set_topmost(&mut self, hwnd: isize);
    /// Move a window without resizing it.
    fn move_to(&mut self, hwnd: isize, x: i32, y: i32);
    /// Show or hide a window, without stealing focus.
    fn show(&mut self, hwnd: isize, visible: bool);
}

/// What a FakeWin was asked to do.
#[derive(Debug, Clone, PartialEq)]
pub enum Call {
    Topmost(isize),
    Move(isize, i32, i32),
    Show(isize, bool),
}

/// A pretend Windows for tests: you set the taskbars and the window in front, it records calls.
#[derive(Debug, Default)]
pub struct FakeWin {
    pub taskbars: Vec<Taskbar>,
    pub foreground: Option<(Rect, Rect)>,
    pub calls: Vec<Call>,
}

impl WinApi for FakeWin {
    fn taskbars(&self) -> Vec<Taskbar> {
        self.taskbars.clone()
    }

    fn foreground(&self) -> Option<(Rect, Rect)> {
        self.foreground
    }

    fn set_topmost(&mut self, hwnd: isize) {
        self.calls.push(Call::Topmost(hwnd));
    }

    fn move_to(&mut self, hwnd: isize, x: i32, y: i32) {
        self.calls.push(Call::Move(hwnd, x, y));
    }

    fn show(&mut self, hwnd: isize, visible: bool) {
        self.calls.push(Call::Show(hwnd, visible));
    }
}

/// A window covers its monitor: a fullscreen or borderless game, a video, a presentation.
pub fn covers_monitor(window: Rect, monitor: Rect) -> bool {
    window.left <= monitor.left && window.top <= monitor.top && window.right >= monitor.right && window.bottom >= monitor.bottom
}

/// Should the bar on `bar_monitor` show, given the window in front (and its monitor)?
pub fn bar_visible(foreground: Option<(Rect, Rect)>, bar_monitor: Rect) -> bool {
    match foreground {
        Some((window, monitor)) => !(monitor == bar_monitor && covers_monitor(window, monitor)),
        None => true,
    }
}

/// The taskbars that get a bar: the primary one, or all of them with "Show on all taskbars".
pub fn taskbars_to_use(taskbars: &[Taskbar], all_taskbars: bool) -> Vec<Taskbar> {
    taskbars.iter().filter(|t| all_taskbars || t.primary).copied().collect()
}
