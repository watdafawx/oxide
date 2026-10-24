//! What the bar draws: pills and dividers, one line or two.

use std::collections::HashMap;

use crate::history::History;
use crate::limits::{Limits, is_warning};
use crate::order::{DividerStyle, Slot};
use crate::reading::Reading;
use crate::stat::StatKind;

/// The latest value of every stat Kinetik could read. A missing key means "could not read".
pub type Snapshot = HashMap<StatKind, f64>;

/// One stat as the bar draws it.
#[derive(Debug, Clone, PartialEq)]
pub struct Pill {
    pub kind: StatKind,
    pub label: &'static str,
    pub text: String,
    pub warning: bool,
}

/// One thing the bar draws, left to right.
#[derive(Debug, Clone, PartialEq)]
pub enum BarItem {
    Pill(Pill),
    Divider(DividerStyle),
}

/// Two-line layout: a column holds one or two pills, stacked; a divider gets its own column.
#[derive(Debug, Clone, PartialEq)]
pub enum Column {
    Pills(Pill, Option<Pill>),
    Divider(DividerStyle),
}

/// Turn the bar order into what the bar draws.
pub fn bar_items(order: &[Slot], snapshot: &Snapshot, limits: &Limits) -> Vec<BarItem> {
    let mut items = Vec::new();
    for slot in order {
        match slot {
            Slot::Stat(kind) => {
                let reading = Reading { kind: *kind, value: snapshot.get(kind).copied() };
                items.push(BarItem::Pill(Pill {
                    kind: *kind,
                    label: kind.label(),
                    text: reading.text(),
                    warning: is_warning(&reading, limits),
                }));
            }
            Slot::Divider(style) => items.push(BarItem::Divider(*style)),
        }
    }
    items
}

/// Pair the pills up for the two-line layout. A divider ends a pair.
pub fn two_line_columns(items: &[BarItem]) -> Vec<Column> {
    let mut columns = Vec::new();
    let mut waiting: Option<Pill> = None;
    for item in items {
        match item {
            BarItem::Pill(pill) => match waiting.take() {
                Some(top) => columns.push(Column::Pills(top, Some(pill.clone()))),
                None => waiting = Some(pill.clone()),
            },
            BarItem::Divider(style) => {
                if let Some(top) = waiting.take() {
                    columns.push(Column::Pills(top, None));
                }
                columns.push(Column::Divider(*style));
            }
        }
    }
    if let Some(top) = waiting {
        columns.push(Column::Pills(top, None));
    }
    columns
}

/// The hover card's line: "min 10% · avg 20% · max 30%", or "no data yet".
pub fn hover_summary(kind: StatKind, history: &History) -> String {
    let Some(s) = history.summary() else {
        return "no data yet".to_string();
    };
    let text = |v: f64| Reading { kind, value: Some(v) }.text();
    format!("min {} · avg {} · max {}", text(s.min), text(s.avg), text(s.max))
}
