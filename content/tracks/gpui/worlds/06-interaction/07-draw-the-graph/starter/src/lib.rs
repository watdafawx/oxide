//! Kinetik Stats: a bar you can click, hover and right-click.

pub mod demo;
pub mod format;
pub mod history;
pub mod limits;
pub mod order;
pub mod pills;
pub mod reading;
pub mod stat;

pub use format::{format_bytes, format_percent, format_speed};
pub use history::{History, Summary};
pub use limits::{Limit, Limits, WarnWhen, is_warning};
pub use order::{DividerStyle, Slot, default_order, move_up};
pub use pills::{BarItem, Column, Pill, Snapshot, bar_items, hover_summary, two_line_columns};
pub use reading::Reading;
pub use stat::{StatKind, Unit};
pub mod bar;
pub mod disk;
pub mod dock;
pub mod docker;
pub mod gpu;
pub mod net;
pub mod ping;
pub mod power;
pub mod real;
pub mod sensor;
pub mod thermal;
pub mod ui;
pub mod uptime;
pub mod view;
pub mod win;
pub mod window;

pub use bar::{SensorRead, StatsBar, clamp_interval, read_sensors, wanted_kinds};
pub use disk::DiskSensor;
pub use dock::{Call, FakeWin, Rect, Taskbar, WinApi, bar_visible, covers_monitor, dock_bar, taskbars_to_use, to_logical};
pub use docker::Docker;
pub use gpu::{GpuSensor, NvidiaGpu};
pub use net::NetSpeedSensor;
pub use ping::PingSensor;
pub use power::BatterySensor;
pub use real::{CpuSensor, RamSensor};
pub use sensor::{Missing, ScriptedSensor, Sensor, missing_text};
pub use thermal::ThermalSensor;
pub use ui::{BarUi, FakeLauncher, LeftClick, Launcher, ProcessUse, RealLauncher};
pub use view::{LabelMode, Look};
pub use win::{RealWin, find_taskbar, hwnd_of};
pub use window::bar_window_options;

use gpui::{Bounds, Context, Div, IntoElement, PathBuilder, Pixels, Point, Render, SharedString, Stateful, Window, canvas, div, point, prelude::*, px, rgb, rgba};

/// What the hover card shows for one stat.
#[derive(Debug, Clone, PartialEq)]
pub struct HoverCard {
    pub title: String,
    pub summary: String,
    pub points: Vec<f64>,
    pub top: Vec<ProcessUse>,
}

/// The card for `kind`, from the bar's history and reasons. The process list is filled when it opens.
pub fn card_for(bar: &StatsBar, kind: StatKind) -> HoverCard {
    HoverCard {
        title: format!("{} {}", kind.label(), bar.text_of(kind).unwrap_or_default()),
        summary: bar.hover_text(kind),
        points: bar.histories.get(&kind).map(|h| h.values().collect()).unwrap_or_default(),
        top: Vec::new(),
    }
}

impl Render for HoverCard {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .debug_selector(|| "hover-card".into())
            .w(px(240.))
            .p_2()
            .gap_1()
            .flex()
            .flex_col()
            .rounded(px(6.))
            .bg(rgb(0x2a2a2a))
            .text_color(rgb(0xffffff))
            .text_sm()
            .child(div().debug_selector(|| "card-title".into()).child(self.title.clone()))
            .child(graph(self.points.clone()))
            .child(div().debug_selector(|| "card-summary".into()).child(self.summary.clone()))
            .children(self.top.iter().enumerate().map(|(i, (name, bytes))| {
                div().debug_selector(move || format!("card-top-{i}")).child(format!("{name}  {}", format_bytes(*bytes)))
            }))
    }
}

/// The last minute as a line, drawn straight onto the window with a canvas.
fn graph(values: Vec<f64>) -> Div {
    div().debug_selector(|| "graph".into()).h(px(48.)).w_full().child(
        canvas(
            |_, _, _| {},
            move |bounds, _, window, _| {
                let points = graph_points(&values, bounds);
                // TODO: start a line 1.5 pixels wide; move to the first point and draw a line to each
                // next one; then build the path and paint it in blue, 0x5ab0ff
                let _ = (points, window);
                todo!("paint the line through the points")
            },
        )
        .size_full(),
    )
}

/// The hover graph's points: oldest on the left edge, newest on the right,
/// the highest value at the top, the lowest at the bottom (a flat line sits in the middle).
pub fn graph_points(values: &[f64], bounds: Bounds<Pixels>) -> Vec<Point<Pixels>> {
    let (min, max) = values.iter().fold((f64::MAX, f64::MIN), |(lo, hi), v| (lo.min(*v), hi.max(*v)));
    let last = values.len().saturating_sub(1).max(1) as f32;
    values
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let x = bounds.left() + bounds.size.width * (i as f32 / last);
            let share = if max > min { ((v - min) / (max - min)) as f32 } else { 0.5 };
            let y = bounds.bottom() - bounds.size.height * share;
            point(x, y)
        })
        .collect()
}

/// Everything a pill does when you touch it.
pub fn interactive_pill(pill: Div, kind: StatKind, bar: &StatsBar, cx: &mut Context<StatsBar>) -> Stateful<Div> {
    let card = card_for(bar, kind);
    pill
        .id(SharedString::from(format!("pill-{kind:?}")))
        .hover(|style| style.bg(rgba(0xffffff14)))
        .on_hover(cx.listener(move |this, hovered: &bool, _, cx| {
            if *hovered {
                this.ui.hovered = Some(kind);
            } else if this.ui.hovered == Some(kind) {
                this.ui.hovered = None;
            }
            cx.notify();
        }))
        .on_click(cx.listener(move |this, _, _, cx| this.on_pill_click(kind, cx)))
        .tooltip(move |_, cx| {
            let card = card.clone();
            cx.new(|_| card).into()
        })
}

/// Things that belong to the whole bar, not one pill. Nothing yet.
pub fn bar_layer(bar: Div, _this: &StatsBar, _cx: &mut Context<StatsBar>) -> Div {
    bar
}

impl StatsBar {
    /// A left click on a pill: remember it, then do what the Left-click action setting says.
    pub fn on_pill_click(&mut self, kind: StatKind, cx: &mut Context<Self>) {
        self.ui.clicked = Some(kind);
        match self.ui.left_click {
            LeftClick::TaskManager => self.ui.launcher.launch("taskmgr"),
            LeftClick::Settings => self.open_settings(cx),
            LeftClick::Nothing => {}
        }
        cx.notify();
    }
}

/// Every sensor this PC has so far; the rest of the bar is scripted.
pub fn live_sensors() -> Vec<Box<dyn Sensor>> {
    let mut sensors: Vec<Box<dyn Sensor>> = vec![
        Box::new(CpuSensor::new()),
        Box::new(RamSensor::new()),
        Box::new(NetSpeedSensor::download()),
        Box::new(NetSpeedSensor::upload()),
        Box::new(DiskSensor::new()),
        Box::new(GpuSensor::usage()),
        Box::new(GpuSensor::temperature()),
        Box::new(ThermalSensor::new()),
        Box::new(BatterySensor),
        Box::new(PingSensor::new()),
    ];
    let real: Vec<StatKind> = sensors.iter().map(|s| s.kind()).collect();
    for kind in [StatKind::CpuUsage, StatKind::CpuTemp, StatKind::GpuUsage, StatKind::GpuTemp, StatKind::RamUsed, StatKind::NetDown, StatKind::NetUp, StatKind::Ping] {
        if !real.contains(&kind) {
            let script = (0..300).map(|i| 20.0 + (i % 7) as f64 * 5.0).collect();
            sensors.push(Box::new(ScriptedSensor::new(kind, script)));
        }
    }
    sensors
}

#[cfg(test)]
#[allow(unused_imports, dead_code)]
mod tests {
    use super::*;
    use gpui::{Modifiers, MouseButton, MouseDownEvent, MouseUpEvent, TestAppContext, VisualTestContext, point, prelude::*, px};
    use std::time::Duration;

    /// A bar with only a scripted CPU sensor, focused, in a test window; three reads in.
    fn open(cx: &mut TestAppContext) -> (gpui::Entity<StatsBar>, &mut VisualTestContext) {
        let (bar, cx) = cx.add_window_view(|window, cx| {
            let sensors: Vec<Box<dyn Sensor>> = vec![Box::new(ScriptedSensor::new(StatKind::CpuUsage, vec![10.0, 20.0, 30.0]))];
            let mut bar = StatsBar::new(sensors);
            bar.ui.launcher = Box::new(FakeLauncher::default());
            bar.ui.top_processes = || vec![("game.exe".into(), 4 << 30), ("browser.exe".into(), 2 << 30)];
            bar.focus_in(window, cx);
            bar.start(cx);
            bar
        });
        for _ in 0..3 {
            cx.executor().advance_clock(Duration::from_secs(1));
            cx.run_until_parked();
        }
        (bar, cx)
    }

    fn centre_of(cx: &mut VisualTestContext, name: &'static str) -> gpui::Point<gpui::Pixels> {
        cx.debug_bounds(name).unwrap_or_else(|| panic!("nothing named {name} was drawn")).center()
    }

    fn hover(cx: &mut VisualTestContext, name: &'static str) {
        let at = centre_of(cx, name);
        cx.simulate_mouse_move(at, None, Modifiers::none());
        cx.executor().advance_clock(Duration::from_millis(600));
        cx.run_until_parked();
    }

    fn move_away(cx: &mut VisualTestContext) {
        cx.simulate_mouse_move(point(px(1500.), px(900.)), None, Modifiers::none());
        cx.executor().advance_clock(Duration::from_millis(600));
        cx.run_until_parked();
    }

    fn right_click(cx: &mut VisualTestContext, name: &'static str) {
        let at = centre_of(cx, name);
        cx.simulate_event(MouseDownEvent { position: at, modifiers: Modifiers::none(), button: MouseButton::Right, click_count: 1, first_mouse: false });
        cx.simulate_event(MouseUpEvent { position: at, modifiers: Modifiers::none(), button: MouseButton::Right, click_count: 1 });
        cx.run_until_parked();
    }

    #[gpui::test]
    fn card_has_graph(cx: &mut TestAppContext) {
        let (_bar, cx) = open(cx);
        hover(cx, "pill-CpuUsage");
        let graph = cx.debug_bounds("graph").expect("the card has no graph");
        let title = cx.debug_bounds("card-title").unwrap();
        let summary = cx.debug_bounds("card-summary").unwrap();
        assert_eq!(graph.size.height, px(48.));
        assert!(title.origin.y < graph.origin.y && graph.origin.y < summary.origin.y, "title, graph, summary, top to bottom");
    }
}
