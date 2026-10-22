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

/// Values a hand-edited or old file may hold, pulled back into range (Kinetik v3.1.2).
pub fn clamp_settings(mut s: Settings) -> Settings {
    s.interval_ms = s.interval_ms.clamp(500, 5000);
    s.font_size = s.font_size.clamp(8.0, 24.0);
    s.opacity = s.opacity.clamp(0.2, 1.0);
    s.gap = s.gap.clamp(0.0, 24.0);
    s
}

/// %APPDATA%\KinetikStats\settings.json (or ./settings.json if APPDATA is not set).
pub fn settings_path() -> PathBuf {
    let base = std::env::var_os("APPDATA").map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."));
    base.join("KinetikStats").join("settings.json")
}

/// Write the settings as JSON: to a temporary file first, then renamed over the old one,
/// so a crash mid-write never leaves half a file.
pub fn save(settings: &Settings, path: &Path) -> io::Result<()> {
    if let Some(folder) = path.parent() {
        fs::create_dir_all(folder)?;
    }
    let json = serde_json::to_string_pretty(settings)?;
    let temp = path.with_extension("tmp");
    fs::write(&temp, json)?;
    fs::rename(&temp, path)
}

/// Read the settings. Out-of-range values are clamped; a file that cannot be read is kept as
/// `.bak` and the defaults are used, so a bad file never stops Kinetik from starting.
pub fn load(path: &Path) -> Settings {
    let Ok(text) = fs::read_to_string(path) else {
        return Settings::default();
    };
    match serde_json::from_str::<Settings>(&text) {
        Ok(settings) => clamp_settings(settings),
        Err(_) => {
            let _ = fs::rename(path, path.with_extension("bak"));
            Settings::default()
        }
    }
}
