//! Keeping the bar docked.

use crate::dock::{Rect, Taskbar, WinApi, dock_bar};

/// Keeps one bar window docked: placed, on top, moved when the taskbar moves, hidden in fullscreen.
pub struct Docker<A: WinApi> {
    pub api: A,
    pub hwnd: isize,
    pub bar_width: i32,
    pub shift_left: i32,
    /// The taskbar the bar was last placed against.
    pub last: Option<Taskbar>,
    pub visible: bool,
}

impl<A: WinApi> Docker<A> {
    pub fn new(api: A, hwnd: isize, bar_width: i32) -> Self {
        Docker { api, hwnd, bar_width, shift_left: 0, last: None, visible: true }
    }

    /// The bar window just opened: put it above the taskbar.
    pub fn on_open(&mut self) {
        self.api.set_topmost(self.hwnd);
    }

    /// The window in front changed. Clicking the taskbar raises it over the bar, so raise the bar
    /// again whenever the taskbar comes to the front.
    pub fn on_foreground_change(&mut self, taskbar_in_front: bool) {
        if taskbar_in_front {
            self.api.set_topmost(self.hwnd);
        }
    }

    /// Check the primary taskbar. When it moved (or appeared), move the bar with it and
    /// give back the new place.
    pub fn redock(&mut self) -> Option<Rect> {
        let taskbar = self.api.taskbars().into_iter().find(|t| t.primary)?;
        if self.last == Some(taskbar) {
            return None;
        }
        self.last = Some(taskbar);
        let place = dock_bar(&taskbar, self.bar_width, self.shift_left)?;
        self.api.move_to(self.hwnd, place.left, place.top);
        Some(place)
    }
}
