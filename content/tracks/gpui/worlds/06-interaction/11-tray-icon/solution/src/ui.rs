//! What the bar remembers about clicks, hovers and menus.

use std::cell::RefCell;
use std::rc::Rc;

use gpui::{Context, FocusHandle, Pixels, Point, Window};

use crate::{StatKind, StatsBar};

/// Starts programs. The real one runs them; tests use one that writes down the name.
pub trait Launcher {
    fn launch(&mut self, program: &str);
}

/// Runs the program, the way Kinetik opens Task Manager on a left click.
pub struct RealLauncher;

impl Launcher for RealLauncher {
    fn launch(&mut self, program: &str) {
        let _ = std::process::Command::new(program).spawn();
    }
}

/// Writes down every program it was asked to start.
#[derive(Clone, Default)]
pub struct FakeLauncher {
    pub launched: Rc<RefCell<Vec<String>>>,
}

impl Launcher for FakeLauncher {
    fn launch(&mut self, program: &str) {
        self.launched.borrow_mut().push(program.to_string());
    }
}

/// Kinetik's "Left-click action" setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LeftClick {
    #[default]
    TaskManager,
    Settings,
    Nothing,
}

/// A process name and the memory it uses, in bytes.
pub type ProcessUse = (String, u64);

/// The bar's interaction state.
pub struct BarUi {
    /// The stat clicked last.
    pub clicked: Option<StatKind>,
    /// The stat under the mouse.
    pub hovered: Option<StatKind>,
    /// Where the right-click menu is open, if it is.
    pub menu_at: Option<Point<Pixels>>,
    /// Things the bar was asked to do, in order: "settings", "exit".
    pub handled: Vec<&'static str>,
    pub left_click: LeftClick,
    pub launcher: Box<dyn Launcher>,
    pub focus: Option<FocusHandle>,
    /// Where the hover card's process list comes from.
    pub top_processes: fn() -> Vec<ProcessUse>,
}

impl Default for BarUi {
    fn default() -> Self {
        BarUi {
            clicked: None,
            hovered: None,
            menu_at: None,
            handled: Vec::new(),
            left_click: LeftClick::default(),
            launcher: Box::new(RealLauncher),
            focus: None,
            top_processes: Vec::new,
        }
    }
}

impl StatsBar {
    /// Give the bar keyboard focus inside its window, so actions reach it.
    pub fn focus_in(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let focus = cx.focus_handle();
        window.focus(&focus);
        self.ui.focus = Some(focus);
    }

    /// Open the Settings window. It arrives in the Settings world; for now the bar notes the request.
    pub fn open_settings(&mut self, cx: &mut Context<Self>) {
        self.ui.handled.push("settings");
        cx.notify();
    }
}
