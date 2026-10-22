//! Keeping the bar docked.

use crate::dock::{Taskbar, WinApi};

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
}
