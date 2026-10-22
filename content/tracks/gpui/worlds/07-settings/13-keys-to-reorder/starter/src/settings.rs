//! Settings: what the user chose, shared through a global, saved as JSON.

#![allow(unused_imports)]

use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use gpui::{App, Context, Global, Subscription};
use serde::{Deserialize, Serialize};

use crate::ui::LeftClick;
use crate::view::{LabelMode, Look};
use crate::{Limits, Sensor, Slot, StatKind, StatsBar, bar_items, default_order};
use crate::*;

/// Everything the Settings window changes, saved as JSON.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub interval_ms: u64,
    pub order: Vec<Slot>,
    pub label_mode: LabelMode,
    pub two_lines: bool,
    pub opacity: f32,
    pub font_size: f32,
    pub gap: f32,
    /// Per-stat colours, 0xRRGGBB.
    pub colours: BTreeMap<StatKind, u32>,
    pub left_click: LeftClick,
    pub ping_address: String,
    pub all_taskbars: bool,
    pub shift_left: i32,
}

impl Default for Settings {
    /// Kinetik's own defaults, as a new install has them.
    fn default() -> Self {
        Settings {
            interval_ms: 1000,
            order: default_order(),
            label_mode: LabelMode::Text,
            two_lines: false,
            opacity: 0.85,
            font_size: 13.0,
            gap: 8.0,
            colours: BTreeMap::new(),
            left_click: LeftClick::TaskManager,
            ping_address: "1.1.1.1".to_string(),
            all_taskbars: false,
            shift_left: 0,
        }
    }
}

/// The one Settings for the whole app, reachable from any view with `cx.global::<Settings>()`.
impl Global for Settings {}

/// Switch a stat on (added at the end of the bar) or off (removed), as the Stats page's toggles do.
pub fn set_stat_on(order: &mut Vec<Slot>, kind: StatKind, on: bool) {
    let present = order.contains(&Slot::Stat(kind));
    if on && !present {
        order.push(Slot::Stat(kind));
    } else if !on {
        order.retain(|slot| *slot != Slot::Stat(kind));
    }
}

impl StatsBar {
    /// A bar set up from the app's Settings.
    pub fn from_settings(sensors: Vec<Box<dyn Sensor>>, cx: &App) -> Self {
        let settings = cx.global::<Settings>();
        let mut bar = StatsBar::new(sensors).with_interval(settings.interval_ms);
        bar.apply_settings(settings);
        bar
    }

    /// Copy what the bar shows from `settings`.
    pub fn apply_settings(&mut self, settings: &Settings) {
        self.look = Look {
            label_mode: settings.label_mode,
            two_lines: settings.two_lines,
            opacity: settings.opacity,
            font_size: settings.font_size,
            gap: settings.gap,
        };
        self.order = settings.order.clone();
        self.colours = settings.colours.clone();
        self.ui.left_click = settings.left_click;
        self.items = bar_items(&self.order, &self.snapshot, &Limits::default());
    }

    /// Redraw with the new settings whenever they change: the live preview.
    pub fn watch_settings(&mut self, cx: &mut Context<Self>) {
        let watch = cx.observe_global::<Settings>(|bar, cx| {
            let settings = cx.global::<Settings>().clone();
            bar.apply_settings(&settings);
            cx.notify();
        });
        self.settings_watch = Some(watch);
    }
}
