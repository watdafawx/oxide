//! The Settings window.

#![allow(unused_imports)]

use std::cell::Cell;
use std::ops::Range;
use std::rc::Rc;

use gpui::{App, Bounds, Context, Div, FocusHandle, IntoElement, KeyBinding, KeyDownEvent, MouseButton, MouseDownEvent, MouseMoveEvent, Pixels, Render, RenderOnce, SharedString, Stateful, TitlebarOptions, UpdateGlobal, Window, WindowBounds, WindowHandle, WindowOptions, actions, canvas, div, prelude::*, px, relative, rgb, size, uniform_list};

use crate::{Settings, Slot, StatKind, move_up, set_stat_on};
use crate::*;

/// Open the Settings window into `slot`, or bring the one already there to the front.
pub fn open_settings_window(cx: &mut App, slot: &mut Option<WindowHandle<SettingsWindow>>) {
    if let Some(handle) = slot
        && handle.update(cx, |_, window, _| window.activate_window()).is_ok()
    {
        return;
    }
    let bounds = Bounds::centered(None, size(px(640.), px(480.)), cx);
    let options = WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(bounds)),
        titlebar: Some(TitlebarOptions { title: Some("Kinetik Stats Settings".into()), ..Default::default() }),
        ..Default::default()
    };
    *slot = cx.open_window(options, |window, cx| cx.new(|cx| SettingsWindow::new(window, cx))).ok();
}

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

/// One page in the sidebar; clicking it shows that page.
pub fn sidebar_row(page: Page, current: Page, cx: &mut Context<SettingsWindow>) -> Stateful<Div> {
    div()
        .id(SharedString::from(format!("page-{page:?}")))
        .debug_selector(move || format!("page-{page:?}"))
        .px_3()
        .py_1()
        .rounded(px(4.))
        .when(page == current, |el| el.bg(rgb(0x2e86de)))
        .child(format!("{page:?}"))
        .on_click(cx.listener(move |this, _, _, cx| {
            this.page = page;
            cx.notify();
        }))
}

/// What a toggle does when clicked: gets the new on/off state.
pub type ToggleHandler = Box<dyn Fn(bool, &mut Window, &mut App) + 'static>;

/// An on/off switch: a rounded track with a knob, left for off, right for on.
#[derive(IntoElement)]
pub struct Toggle {
    id: SharedString,
    on: bool,
    on_toggle: ToggleHandler,
}

impl Toggle {
    pub fn new(id: impl Into<SharedString>, on: bool, on_toggle: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        Toggle { id: id.into(), on, on_toggle: Box::new(on_toggle) }
    }
}

impl RenderOnce for Toggle {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let on = self.on;
        let on_toggle = self.on_toggle;
        let id = self.id.clone();
        div()
            .id(self.id)
            .debug_selector(move || id.to_string())
            .w(px(36.))
            .h(px(20.))
            .p(px(2.))
            .flex()
            .rounded_full()
            .bg(if on { rgb(0x5ab0ff) } else { rgb(0x555555) })
            .when(on, |el| el.justify_end())
            .child(div().size(px(16.)).rounded_full().bg(rgb(0xffffff)))
            .on_click(move |_, window, cx| on_toggle(!on, window, cx))
    }
}

/// The slider value at `x` on a track from `left`, `width` wide: clamped to the track,
/// mapped onto min..=max, snapped to `step`.
pub fn slider_value(x: f32, left: f32, width: f32, min: f32, max: f32, step: f32) -> f32 {
    let share = ((x - left) / width).clamp(0.0, 1.0);
    let raw = min + share * (max - min);
    let snapped = min + ((raw - min) / step).round() * step;
    snapped.clamp(min, max)
}

impl SettingsWindow {
    /// Set the gap from a mouse x on the slider's track.
    pub fn set_gap_at(&mut self, x: Pixels, cx: &mut Context<Self>) {
        let track = self.gap_track.get();
        let gap = slider_value(f32::from(x), f32::from(track.left()), f32::from(track.size.width), 0.0, 24.0, 1.0);
        Settings::update_global(cx, |s, _| s.gap = gap);
        cx.notify();
    }
}

/// One row of the Arrange list: the stat's label or a divider; click selects it.
pub fn arrange_row(index: usize, slot: Slot, selected: bool, cx: &mut Context<SettingsWindow>) -> Stateful<Div> {
    let text = match slot {
        Slot::Stat(kind) => format!("{}  {kind:?}", kind.label()),
        Slot::Divider(style) => format!("— divider ({style:?})"),
    };
    div()
        .id(("row", index))
        .debug_selector(move || format!("row-{index}"))
        .h(px(28.))
        .px_2()
        .flex()
        .items_center()
        .when(selected, |el| el.bg(rgb(0x2e86de)))
        .child(text)
        .on_click(cx.listener(move |this, _, window, cx| {
            this.selected = index;
            window.focus(&this.focus);
            cx.notify();
        }))
}

/// The Arrange list, every row at once (a long list comes later).
pub fn arrange_list(this: &SettingsWindow, cx: &mut Context<SettingsWindow>) -> impl IntoElement {
    let order = cx.global::<Settings>().order.clone();
    let selected = this.selected;
    div().flex().flex_col().children(order.into_iter().enumerate().map(|(i, slot)| arrange_row(i, slot, i == selected, cx)))
}

impl Render for SettingsWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let order = cx.global::<Settings>().order.clone();
        let _ = &order;
        let content = match self.page {
            Page::Stats => div().flex().flex_col().gap_2().child("Stats")
                .children(crate::stat::ALL.into_iter().map(|kind| {
                    let on = order.contains(&Slot::Stat(kind));
                    div().flex().gap_2().items_center().child(Toggle::new(format!("toggle-{kind:?}"), on, move |on, _, cx| {
                        Settings::update_global(cx, |s, _| set_stat_on(&mut s.order, kind, on));
                    })).child(format!("{} {kind:?}", kind.label()))
                })),
            Page::Arrange => div().flex().flex_col().gap_2().child("Arrange").child(arrange_list(self, cx)),
            Page::Appearance => div().flex().flex_col().gap_2().child("Appearance")
                .child(div().flex().gap_2().items_center().child("Gap").child(gap_slider(self, cx))),
        };
        let page = self.page;
        div()
            .debug_selector(|| "settings".into())
            .size_full()
            .flex()
            .bg(rgb(0x1e1e1e))
            .text_color(rgb(0xffffff))
            .child(div().w(px(140.)).p_2().flex().flex_col().gap_1().children(PAGES.map(|p| sidebar_row(p, page, cx))))
            .child(div().debug_selector(move || format!("content-{page:?}")).flex_1().p_4().child(content))
    }
}
