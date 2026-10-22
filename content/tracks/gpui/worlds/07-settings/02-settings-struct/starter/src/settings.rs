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

/// Switch a stat on (added at the end of the bar) or off (removed), as the Stats page's toggles do.
pub fn set_stat_on(order: &mut Vec<Slot>, kind: StatKind, on: bool) {
    let present = order.contains(&Slot::Stat(kind));
    if on && !present {
        order.push(Slot::Stat(kind));
    } else if !on {
        order.retain(|slot| *slot != Slot::Stat(kind));
    }
}
