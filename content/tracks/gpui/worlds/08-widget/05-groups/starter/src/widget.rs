//! The desktop widget.

#![allow(unused_imports, unused_variables)]

use std::cell::RefCell;

use gpui::{App, Context, Div, Entity, IntoElement, MouseButton, PathBuilder, Render, Stateful, Subscription, Window, WindowBackgroundAppearance, WindowBounds, WindowHandle, WindowKind, WindowOptions, actions, canvas, div, prelude::*, px, relative, rgb, rgba};

use crate::*;

pub const WIDGET_WIDTH: i32 = 300;
pub const WIDGET_HEIGHT: i32 = 420;

actions!(widget, [ToggleWidget]);

/// The desktop widget: the bar's readings, grouped, with bars and graphs.
pub struct WidgetView {
    pub bar: Entity<StatsBar>,
    /// Per-thread usage for the core bars.
    pub cores: Vec<f32>,
    /// Where core usage comes from: this PC, or a test's fixed list.
    pub core_source: fn() -> Vec<f32>,
    /// How many drags were started from the header.
    pub moves_started: u32,
    /// Starts a native window drag. Tests swap it for one that does nothing.
    pub start_move: fn(&mut Window),
    _watch: Subscription,
    _settings: Subscription,
}

impl WidgetView {
    pub fn new(bar: Entity<StatsBar>, _window: &mut Window, cx: &mut Context<Self>) -> Self {
        let watch = cx.observe(&bar, |this, _, cx| {
            this.cores = (this.core_source)();
            cx.notify();
        });
        let settings = cx.observe_global::<Settings>(|_, cx| cx.notify());
        WidgetView {
            bar,
            cores: Vec::new(),
            core_source: real_cores,
            moves_started: 0,
            start_move: |window| window.start_window_move(),
            _watch: watch,
            _settings: settings,
        }
    }
}

thread_local! {
    static CORE_READER: RefCell<sysinfo::System> = RefCell::new(sysinfo::System::new());
}

/// This PC's per-thread usage, measured since the last call.
pub fn real_cores() -> Vec<f32> {
    CORE_READER.with(|system| {
        let mut system = system.borrow_mut();
        system.refresh_cpu_usage();
        system.cpus().iter().map(|cpu| cpu.cpu_usage()).collect()
    })
}

impl StatsBar {
    /// Open or close the desktop widget. Deferred: the widget reads the bar while it draws,
    /// and the bar is busy until this method returns.
    pub fn toggle_widget(&mut self, cx: &mut Context<Self>) {
        let mut slot = self.ui.widget_window.take();
        let bar = cx.entity();
        cx.defer(move |cx| {
            toggle_widget_window(cx, &mut slot, bar.clone());
            bar.update(cx, |this, _| this.ui.widget_window = slot);
        });
    }
}

/// A small line graph of a history.
pub fn widget_graph(kind: StatKind, values: Vec<f64>) -> Div {
    div().debug_selector(move || format!("graph-{kind:?}")).h(px(24.)).w_full().child(
        canvas(
            |_, _, _| {},
            move |bounds, _, window, _| {
                let points = graph_points(&values, bounds);
                let mut line = PathBuilder::stroke(px(1.));
                for (i, p) in points.iter().enumerate() {
                    if i == 0 { line.move_to(*p) } else { line.line_to(*p) }
                }
                if let Ok(path) = line.build() {
                    window.paint_path(path, rgb(0x5ab0ff));
                }
            },
        )
        .size_full(),
    )
}

/// The widget's headings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Group {
    Processor,
    Graphics,
    Memory,
    Network,
    Storage,
    Battery,
}

/// The headings, top to bottom.
pub const GROUPS: [Group; 6] = [Group::Processor, Group::Graphics, Group::Memory, Group::Network, Group::Storage, Group::Battery];

/// How full a stat's usage bar is, 0 to 1, or None for stats without a bar.
pub fn bar_share(kind: StatKind, value: f64) -> Option<f32> {
    let max = match kind.unit() {
        Unit::Percent => 100.0,
        Unit::Celsius => 110.0,
        _ => return None,
    };
    Some((value / max).clamp(0.0, 1.0) as f32)
}

/// Open the widget, or close it if it is open: the Desktop widget menu row.
pub fn toggle_widget_window(cx: &mut App, slot: &mut Option<WindowHandle<WidgetView>>, bar: Entity<StatsBar>) {
    if let Some(handle) = slot.take()
        && handle.update(cx, |_, window, _| window.remove_window()).is_ok()
    {
        return;
    }
    let settings = cx.global::<Settings>().widget.clone();
    let screen = cx.primary_display().map(|d| d.bounds()).unwrap_or_default();
    let work = Rect::new(0, 0, f32::from(screen.size.width) as i32, f32::from(screen.size.height) as i32);
    let place = place_widget(settings.position, WIDGET_WIDTH, WIDGET_HEIGHT, work);
    let options = WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(to_logical(place, 1.0))),
        titlebar: None,
        kind: WindowKind::Normal,
        focus: false,
        window_background: WindowBackgroundAppearance::Transparent,
        ..Default::default()
    };
    *slot = cx.open_window(options, |window, cx| cx.new(|cx| WidgetView::new(bar, window, cx))).ok();
}

/// The widget's title strip: press and drag it to move the widget (unless locked).
pub fn widget_header(cx: &mut Context<WidgetView>) -> Stateful<Div> {
    div()
        .id("widget-header")
        .debug_selector(|| "widget-header".into())
        .px_2()
        .py_1()
        .text_xs()
        .text_color(rgb(0x9a9a9a))
        .child("Kinetik Stats")
        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, window, cx| {
            if cx.global::<Settings>().widget.locked {
                return;
            }
            this.moves_started += 1;
            (this.start_move)(window);
        }))
}

/// Within `margin` pixels of an edge of the work area, the widget jumps flush to it.
pub fn snap_to_edges(window: Rect, work: Rect, margin: i32) -> Rect {
    let mut r = window;
    let (w, h) = (window.width(), window.height());
    if (window.left - work.left).abs() <= margin {
        r.left = work.left;
    } else if (work.right - window.right).abs() <= margin {
        r.left = work.right - w;
    }
    if (window.top - work.top).abs() <= margin {
        r.top = work.top;
    } else if (work.bottom - window.bottom).abs() <= margin {
        r.top = work.bottom - h;
    }
    r.right = r.left + w;
    r.bottom = r.top + h;
    r
}

/// Where the widget opens: where it was left, if enough of it would be on screen;
/// else the top-right corner, 24 pixels in.
pub fn place_widget(saved: Option<(i32, i32)>, width: i32, height: i32, work: Rect) -> Rect {
    if let Some((x, y)) = saved {
        let place = Rect::new(x, y, x + width, y + height);
        if place.intersect(&work).is_some_and(|seen| seen.width() >= 40 && seen.height() >= 40) {
            return place;
        }
    }
    let right = work.right - 24;
    Rect::new(right - width, work.top + 24, right, work.top + 24 + height)
}

/// WS_EX_LAYERED | WS_EX_TRANSPARENT: together they let clicks fall through a window.
pub const CLICK_THROUGH: u32 = 0x0008_0000 | 0x0000_0020;

/// A window's extended style with click-through switched on or off, every other bit kept.
pub fn click_through_style(style: u32, on: bool) -> u32 {
    if on { style | CLICK_THROUGH } else { style & !CLICK_THROUGH }
}

impl Render for WidgetView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let content = {
            let bar = self.bar.read(cx);
            let (show_bars, show_graphs) = (true, true);
            let stats: Vec<StatKind> = bar.order.iter().filter_map(|s| match s { Slot::Stat(k) => Some(*k), _ => None }).collect();
            let row = |kind: StatKind| {
                let text = bar.text_of(kind).unwrap_or_else(|| "--".into());
                let value = bar.snapshot.get(&kind).copied();
                let history: Vec<f64> = bar.histories.get(&kind).map(|h| h.values().collect()).unwrap_or_default();
                div()
                    .debug_selector(move || format!("row-{kind:?}"))
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(div().flex().justify_between().child(kind.label()).child(text))
            };
            GROUPS.iter().filter_map(|group| {
                let mine: Vec<StatKind> = stats.iter().copied().filter(|k| group_of(*k) == *group).collect();
                if mine.is_empty() {
                    return None;
                }
                let g = *group;
                Some(
                    div()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .child(div().debug_selector(move || format!("heading-{g:?}")).text_xs().text_color(rgb(0x9a9a9a)).child(format!("{g:?}")))
                        .children(mine.into_iter().map(&row)),
                )
            }).collect::<Vec<_>>()
        };
        div()
            .debug_selector(|| "widget".into())
            .size_full()
            .flex()
            .flex_col()
            .gap_2()
            .p_2()
            .rounded(px(10.))
            .bg(rgba(0x1e1e1ef2))
            .text_color(rgb(0xffffff))
            .text_sm()
            .child(widget_header(cx))
            .children(content)
    }
}
