//! Clicks, hovers, the hover card, actions, the right-click menu and the tray (the Interaction world).

use std::cmp::Reverse;

use gpui::{Action, Bounds, Context, Div, IntoElement, MouseButton, MouseDownEvent, PathBuilder, Pixels, Point, Render, SharedString, Stateful, Window, actions, anchored, canvas, deferred, div, point, prelude::*, px, rgb, rgba};
use tray_icon::{MouseButtonState, TrayIconEvent};

use crate::ui::{LeftClick, ProcessUse};
use crate::{StatKind, StatsBar, ToggleOverlay, ToggleWidget, format_bytes};

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

/// Everything a pill does when you touch it.
pub fn interactive_pill(pill: Div, kind: StatKind, bar: &StatsBar, cx: &mut Context<StatsBar>) -> Stateful<Div> {
    let card = card_for(bar, kind);
    let top_processes = bar.ui.top_processes;
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
            let card = HoverCard { top: (top_processes)(), ..card.clone() };
            cx.new(|_| card).into()
        })
        .on_mouse_down(MouseButton::Right, cx.listener(|this, event: &MouseDownEvent, _, cx| {
            this.ui.menu_at = Some(event.position);
            cx.notify();
        }))
}

/// Things that belong to the whole bar, not one pill: focus, actions, the right-click menu.
pub fn bar_layer(bar: Div, this: &StatsBar, cx: &mut Context<StatsBar>) -> Div {
    let bar = match &this.ui.focus {
        Some(focus) => bar.track_focus(focus),
        None => bar,
    };
    bar
        .on_action(cx.listener(|this, _: &OpenSettings, _, cx| this.open_settings(cx)))
        .on_action(cx.listener(|this, _: &OpenTaskManager, _, _| this.ui.launcher.launch("taskmgr")))
        .on_action(cx.listener(|this, _: &ToggleWidget, _, cx| this.toggle_widget(cx)))
        .on_action(cx.listener(|this, _: &ToggleOverlay, _, cx| this.toggle_overlay(cx)))
        .on_action(cx.listener(|this, _: &Exit, _, cx| {
            this.ui.handled.push("exit");
            cx.quit();
        }))
        .when_some(this.ui.menu_at, |el, at| el.child(deferred(anchored().position(at).child(menu(cx)))))
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
        .child(menu_row("widget", "Desktop widget", ToggleWidget, cx))
        .child(menu_row("overlay", "Game overlay", ToggleOverlay, cx))
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
