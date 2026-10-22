//! Where the bar goes.

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
