//! The dashboard window.

#![allow(unused_imports, unused_variables)]

use gpui::{App, Bounds, Context, Div, Entity, IntoElement, PathBuilder, Pixels, Point, Render, RenderOnce, Subscription, TitlebarOptions, Window, WindowBounds, WindowHandle, WindowOptions, actions, canvas, div, point, prelude::*, px, rgb, size};

use crate::stat::ALL;
use crate::*;

/// The colours of one dashboard theme, as 0xRRGGBB.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Theme {
    pub background: u32,
    pub card: u32,
    pub text: u32,
    pub accent: u32,
    pub monospace: bool,
}

/// A card's three numbers, written in the stat's unit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardNumbers {
    pub now: String,
    pub avg: String,
    pub peak: String,
}

/// One dashboard card: the stat's name, its value big, average and peak small, a gauge for percentages.
#[derive(IntoElement)]
pub struct Card {
    pub kind: StatKind,
    pub numbers: CardNumbers,
    pub share: Option<f32>,
    pub history: Vec<f64>,
    pub theme: Theme,
    pub text_size: Pixels,
}

impl RenderOnce for Card {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let kind = self.kind;
        let t = self.theme;
        div()
            .debug_selector(move || format!("card-{kind:?}"))
            .flex()
            .flex_col()
            .gap_1()
            .p_3()
            .rounded(px(8.))
            .bg(rgb(t.card))
            .text_color(rgb(t.text))
            .when(t.monospace, |el| el.font_family("Consolas"))
            .child(div().text_sm().child(format!("{} {kind:?}", kind.label())))
            .child(div().debug_selector(move || format!("now-{kind:?}")).text_size(self.text_size).child(self.numbers.now))
            .child(
                div()
                    .flex()
                    .gap_3()
                    .text_xs()
                    .child(div().debug_selector(move || format!("avg-{kind:?}")).child(format!("avg {}", self.numbers.avg)))
                    .child(div().debug_selector(move || format!("peak-{kind:?}")).child(format!("peak {}", self.numbers.peak))),
            )
            .when(!self.history.is_empty(), |el| el.child(widget_graph(kind, self.history)))
    }
}


actions!(dashboard, [ToggleDashboard]);

/// The dashboard window: a grid of cards, one per visible stat.
pub struct DashboardView {
    pub bar: Entity<StatsBar>,
    _watch: Subscription,
    _settings: Subscription,
}

impl DashboardView {
    pub fn new(bar: Entity<StatsBar>, cx: &mut Context<Self>) -> Self {
        let watch = cx.observe(&bar, |_, _, cx| cx.notify());
        let settings = cx.observe_global::<Settings>(|_, cx| cx.notify());
        DashboardView { bar, _watch: watch, _settings: settings }
    }
}

/// Open the dashboard, or close it if it is open.
pub fn toggle_dashboard_window(cx: &mut App, slot: &mut Option<WindowHandle<DashboardView>>, bar: Entity<StatsBar>) {
    if let Some(handle) = slot.take()
        && handle.update(cx, |_, window, _| window.remove_window()).is_ok()
    {
        return;
    }
    let bounds = Bounds::centered(None, size(px(1400.), px(820.)), cx);
    let options = WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(bounds)),
        titlebar: Some(TitlebarOptions { title: Some("Kinetik Stats Dashboard".into()), ..Default::default() }),
        ..Default::default()
    };
    *slot = cx.open_window(options, |_, cx| cx.new(|cx| DashboardView::new(bar, cx))).ok();
}

impl StatsBar {
    /// Open or close the dashboard (deferred: the dashboard reads the bar while it draws).
    pub fn toggle_dashboard(&mut self, cx: &mut Context<Self>) {
        let mut slot = self.ui.dashboard_window.take();
        let bar = cx.entity();
        cx.defer(move |cx| {
            toggle_dashboard_window(cx, &mut slot, bar.clone());
            bar.update(cx, |this, _| this.ui.dashboard_window = slot);
        });
    }
}
