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

/// How wide a card may get before another column fits.
pub const CARD_MIN_WIDTH: f32 = 320.0;

/// How many columns of cards fit in the window: at least 1, at most 6.
pub fn columns_for(width: Pixels) -> usize {
    ((f32::from(width) / CARD_MIN_WIDTH) as usize).clamp(1, 6)
}

/// The big number's size on a card: a tenth of the card's width, between 14 and 32 pixels.
pub fn card_text_size(card_width: Pixels) -> Pixels {
    px((f32::from(card_width) / 10.0).clamp(14.0, 32.0))
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
            .when_some(self.share, |el, share| el.child(gauge(kind, share, t.accent)))
            .when(!self.history.is_empty(), |el| el.child(widget_graph(kind, self.history)))
    }
}

/// A half-circle gauge filled to `share`, drawn as short line segments.
fn gauge(kind: StatKind, share: f32, colour: u32) -> Div {
    div().debug_selector(move || format!("gauge-{kind:?}")).h(px(48.)).w(px(96.)).child(
        canvas(
            |_, _, _| {},
            move |bounds, _, window, _| {
                let center = point(bounds.center().x, bounds.bottom());
                let radius = bounds.size.height - px(4.);
                for (part, until) in [(rgb(0x444444), 1.0), (rgb(colour), share)] {
                    let mut line = PathBuilder::stroke(px(5.));
                    let steps = 32;
                    for i in 0..=steps {
                        let p = gauge_point(center, radius, until * i as f32 / steps as f32);
                        if i == 0 { line.move_to(p) } else { line.line_to(p) }
                    }
                    if let Ok(path) = line.build() {
                        window.paint_path(path, part);
                    }
                }
            },
        )
        .size_full(),
    )
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
                    share: value.and_then(|v| bar_share(kind, v)),
                    history: bar.histories.get(&kind).map(|h| h.values().collect()).unwrap_or_default(),
                    theme: t,
                    text_size: card_text_size(card_width),
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
