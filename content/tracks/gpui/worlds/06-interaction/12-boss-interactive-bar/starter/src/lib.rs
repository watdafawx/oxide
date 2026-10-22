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

use gpui::{Action, Bounds, Context, Div, IntoElement, MouseButton, MouseDownEvent, PathBuilder, Pixels, Point, Render, SharedString, Stateful, Window, actions, anchored, canvas, deferred, div, point, prelude::*, px, rgb, rgba};

actions!(kinetik, [OpenSettings, OpenTaskManager, Exit]);

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
                let mut line = PathBuilder::stroke(px(1.5));
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

use std::cmp::Reverse;
/// The five processes using the most memory, biggest first.
pub fn top_five(mut processes: Vec<ProcessUse>) -> Vec<ProcessUse> {
    processes.sort_by_key(|p| Reverse(p.1));
    processes.truncate(5);
    processes
}

/// This PC's processes by memory, through sysinfo. Called only when a hover card opens.
pub fn real_top_processes() -> Vec<ProcessUse> {
    let mut system = sysinfo::System::new();
    system.refresh_processes(sysinfo::ProcessesToUpdate::All, true);
    let all = system.processes().values().map(|p| (p.name().to_string_lossy().into_owned(), p.memory())).collect();
    top_five(all)
}

use tray_icon::{MouseButtonState, TrayIconEvent};

/// What a tray icon event asks the bar to do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayAction {
    OpenSettings,
    Exit,
}

/// Clicking the tray icon (left button, on release) opens Settings, like Kinetik.
pub fn tray_click_action(event: &TrayIconEvent) -> Option<TrayAction> {
    match event {
        TrayIconEvent::Click { button: tray_icon::MouseButton::Left, button_state: MouseButtonState::Up, .. } => {
            Some(TrayAction::OpenSettings)
        }
        _ => None,
    }
}

/// The tray menu's rows, by id.
pub fn tray_menu_action(id: &str) -> Option<TrayAction> {
    match id {
        "settings" => Some(TrayAction::OpenSettings),
        "exit" => Some(TrayAction::Exit),
        _ => None,
    }
}

/// Everything a pill does when you touch it: an id, a lighter look and `ui.hovered` under the
/// mouse, the left-click action, the hover card (with processes) and the right-click menu.
pub fn interactive_pill(pill: Div, kind: StatKind, bar: &StatsBar, cx: &mut Context<StatsBar>) -> Stateful<Div> {
    let _ = (kind, bar, cx);
    todo!("id, hover + on_hover, on_click, tooltip with card_for and top processes, right-button on_mouse_down")
}

/// The whole bar: focus, the three actions, and the right-click menu while it is open.
pub fn bar_layer(bar: Div, this: &StatsBar, cx: &mut Context<StatsBar>) -> Div {
    let _ = (this, cx);
    let _ = bar;
    todo!("track_focus, on_action for OpenSettings / OpenTaskManager / Exit, the deferred anchored menu")
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

/// The right-click menu: closes when you click anywhere else.
fn menu(cx: &mut Context<StatsBar>) -> Div {
    div()
        .debug_selector(|| "menu".into())
        .w(px(160.))
        .py_1()
        .rounded(px(6.))
        .bg(rgb(0x2a2a2a))
        .text_color(rgb(0xffffff))
        .on_mouse_down_out(cx.listener(|this, _, _, cx| {
            this.ui.menu_at = None;
            cx.notify();
        }))
        .child(menu_row("settings", "Settings…", OpenSettings, cx))
        .child(menu_row("exit", "Exit", Exit, cx))
}

/// One menu row: its label, and the action a click dispatches.
fn menu_row(id: &'static str, label: &'static str, action: impl Action, cx: &mut Context<StatsBar>) -> Stateful<Div> {
    div()
        .id(label)
        .debug_selector(move || format!("menu-{id}"))
        .px_2()
        .hover(|s| s.bg(rgb(0x3a3a3a)))
        .child(label)
        .on_click(cx.listener(move |this, _, window, cx| {
            this.ui.menu_at = None;
            window.dispatch_action(action.boxed_clone(), cx);
            cx.notify();
        }))
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
    fn left_click_runs_action(cx: &mut TestAppContext) {
        let (bar, cx) = open(cx);
        let launcher = FakeLauncher::default();
        let launched = launcher.launched.clone();
        bar.update(cx, |bar, _| bar.ui.launcher = Box::new(launcher));
        let at = centre_of(cx, "pill-CpuUsage");
        cx.simulate_click(at, Modifiers::none());
        assert_eq!(*launched.borrow(), vec!["taskmgr".to_string()]);
        bar.update(cx, |bar, _| bar.ui.left_click = LeftClick::Settings);
        cx.simulate_click(at, Modifiers::none());
        assert_eq!(bar.read_with(cx, |bar, _| bar.ui.handled.clone()), vec!["settings"]);
    }

    #[gpui::test]
    fn hover_card_appears(cx: &mut TestAppContext) {
        let (bar, cx) = open(cx);
        hover(cx, "pill-CpuUsage");
        assert_eq!(bar.read_with(cx, |bar, _| bar.ui.hovered), Some(StatKind::CpuUsage));
        assert!(cx.debug_bounds("hover-card").is_some());
        assert!(cx.debug_bounds("graph").is_some());
    }

    #[gpui::test]
    fn card_shows_summary(cx: &mut TestAppContext) {
        let (bar, cx) = open(cx);
        let card = bar.read_with(cx, |bar, _| card_for(bar, StatKind::CpuUsage));
        assert_eq!(card.summary, "min 10% · avg 20% · max 30%");
        assert_eq!(card.points, vec![10.0, 20.0, 30.0]);
        assert_eq!(card.title, "CPU 30%");
    }

    #[gpui::test]
    fn card_lists_top_processes(cx: &mut TestAppContext) {
        let (_bar, cx) = open(cx);
        hover(cx, "pill-CpuUsage");
        assert!(cx.debug_bounds("card-top-0").is_some(), "game.exe is listed");
        assert!(cx.debug_bounds("card-top-1").is_some(), "browser.exe is listed");
        assert!(cx.debug_bounds("card-top-2").is_none());
    }

    #[gpui::test]
    fn menu_opens_on_right_click(cx: &mut TestAppContext) {
        let (bar, cx) = open(cx);
        right_click(cx, "pill-RamUsed");
        assert!(bar.read_with(cx, |bar, _| bar.ui.menu_at.is_some()));
        assert!(cx.debug_bounds("menu-exit").is_some());
    }

    #[gpui::test]
    fn menu_exit_quits(cx: &mut TestAppContext) {
        let (bar, cx) = open(cx);
        right_click(cx, "pill-RamUsed");
        let exit = centre_of(cx, "menu-exit");
        cx.simulate_click(exit, Modifiers::none());
        assert_eq!(bar.read_with(cx, |bar, _| bar.ui.handled.clone()), vec!["exit"]);
    }

    #[test]
    fn tray_open_settings() {
        let event = TrayIconEvent::Click {
            id: tray_icon::TrayIconId::new("kinetik"),
            position: tray_icon::dpi::PhysicalPosition::new(0.0, 0.0),
            rect: tray_icon::Rect::default(),
            button: tray_icon::MouseButton::Left,
            button_state: MouseButtonState::Up,
        };
        assert_eq!(tray_click_action(&event), Some(TrayAction::OpenSettings));
        assert_eq!(tray_menu_action("exit"), Some(TrayAction::Exit));
    }
}
