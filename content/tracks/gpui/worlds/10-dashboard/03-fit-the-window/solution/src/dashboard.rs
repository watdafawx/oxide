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

/// Now, average and peak for one stat, from the bar's snapshot and history.
pub fn card_numbers(bar: &StatsBar, kind: StatKind) -> CardNumbers {
    let text = |value: Option<f64>| Reading { kind, value }.text();
    let summary = bar.histories.get(&kind).and_then(|h| h.summary());
    CardNumbers {
        now: text(bar.snapshot.get(&kind).copied()),
        avg: text(summary.map(|s| s.avg)),
        peak: text(summary.map(|s| s.max)),
    }
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

impl Render for DashboardView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let settings = cx.global::<Settings>().dashboard.clone();
        let width = window.viewport_size().width;
        let columns = columns_for(width);
        let card_width = (width - px(32.)) / columns as f32;
        let t = Theme { background: 0x15171c, card: 0x22252d, text: 0xf2f2f2, accent: 0x5ab0ff, monospace: false };
        let _ = (&settings, card_width);
        let bar = self.bar.read(cx);
        let cards: Vec<Card> = ALL.to_vec()
            .into_iter()
            .map(|kind| {
                let value = bar.snapshot.get(&kind).copied();
                Card {
                    kind,
                    numbers: card_numbers(bar, kind),
                    share: None,
                    history: bar.histories.get(&kind).map(|h| h.values().collect()).unwrap_or_default(),
                    theme: t,
                    text_size: px(24.),
                }
            })
            .collect();
        div()
            .debug_selector(|| "dashboard".into())
            .size_full()
            .p_4()
            .bg(rgb(t.background))
            .grid()
            .grid_cols(columns as u16)
            .gap_4()
            .children(cards)
    }
}
