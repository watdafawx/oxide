//! The Settings window.

#![allow(unused_imports)]

use std::cell::Cell;
use std::ops::Range;
use std::rc::Rc;

use gpui::{App, Bounds, Context, Div, FocusHandle, IntoElement, KeyBinding, KeyDownEvent, MouseButton, MouseDownEvent, MouseMoveEvent, Pixels, Render, RenderOnce, SharedString, Stateful, TitlebarOptions, UpdateGlobal, Window, WindowBounds, WindowHandle, WindowOptions, actions, canvas, div, prelude::*, px, relative, rgb, size, uniform_list};

use crate::{Settings, Slot, StatKind, move_up, set_stat_on};
use crate::*;

/// The pages of the Settings window, down the left side.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Stats,
    Arrange,
    Appearance,
}

pub const PAGES: [Page; 3] = [Page::Stats, Page::Arrange, Page::Appearance];

/// The Settings window. Everything it changes goes into the Settings global.
pub struct SettingsWindow {
    pub page: Page,
    /// Focus for the Arrange list, so Alt+Up reaches it.
    pub focus: FocusHandle,
    pub address_focus: FocusHandle,
    /// The selected Arrange row.
    pub selected: usize,
    /// Where the Gap slider's track was drawn, for turning a mouse x into a value.
    pub gap_track: Rc<Cell<Bounds<Pixels>>>,
}

impl SettingsWindow {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus = cx.focus_handle();
        window.focus(&focus);
        SettingsWindow {
            page: Page::Stats,
            focus,
            address_focus: cx.focus_handle(),
            selected: 0,
            gap_track: Rc::new(Cell::new(Bounds::default())),
        }
    }
}

impl Render for SettingsWindow {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().debug_selector(|| "settings".into()).size_full().p_4().bg(rgb(0x1e1e1e)).text_color(rgb(0xffffff)).child("Settings")
    }
}
