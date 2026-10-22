//! Kinetik's stat model.

pub mod format;
pub mod limits;
pub mod reading;
pub mod stat;

pub use format::{format_bytes, format_percent, format_speed};
pub use limits::{Limit, Limits, WarnWhen, is_warning};
pub use reading::Reading;
pub use stat::{ALL, StatKind, Unit};
pub mod history;

pub use history::{History, Summary};
pub mod order;

pub use order::{DividerStyle, Slot, default_order, move_up};

use std::collections::HashMap;

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

#[cfg(test)]
mod tests {
    use super::*;

    fn pill(kind: StatKind, text: &str, warning: bool) -> BarItem {
        BarItem::Pill(Pill { kind, label: kind.label(), text: text.to_string(), warning })
    }

    fn snapshot() -> Snapshot {
        Snapshot::from([
            (StatKind::CpuUsage, 12.6),
            (StatKind::CpuTemp, 90.0),
            (StatKind::RamUsed, 8_589_934_592.0),
            (StatKind::NetDown, 3_565_158.0),
            (StatKind::Battery, 15.0),
        ])
    }

    #[test]
    fn formats_each_kind() {
        let order = [Slot::Stat(StatKind::CpuUsage), Slot::Stat(StatKind::RamUsed), Slot::Stat(StatKind::NetDown)];
        let items = bar_items(&order, &snapshot(), &Limits::default());
        assert_eq!(
            items,
            vec![
                pill(StatKind::CpuUsage, "13%", false),
                pill(StatKind::RamUsed, "8.0 GB", false),
                pill(StatKind::NetDown, "↓ 3.4 MB/s", false),
            ]
        );
    }

    #[test]
    fn missing_shows_dashes() {
        let items = bar_items(&[Slot::Stat(StatKind::Ping)], &snapshot(), &Limits::default());
        assert_eq!(items, vec![pill(StatKind::Ping, "--", false)]);
    }

    #[test]
    fn hot_cpu_is_flagged() {
        let items = bar_items(&[Slot::Stat(StatKind::CpuTemp)], &snapshot(), &Limits::default());
        assert_eq!(items, vec![pill(StatKind::CpuTemp, "90°C", true)]);
    }

    #[test]
    fn low_battery_is_flagged() {
        let items = bar_items(&[Slot::Stat(StatKind::Battery)], &snapshot(), &Limits::default());
        assert_eq!(items, vec![pill(StatKind::Battery, "15%", true)]);
    }

    #[test]
    fn dividers_pass_through() {
        let order = [Slot::Stat(StatKind::CpuUsage), Slot::Divider(DividerStyle::Dot), Slot::Stat(StatKind::Ping)];
        let items = bar_items(&order, &snapshot(), &Limits::default());
        assert_eq!(items.len(), 3);
        assert_eq!(items[1], BarItem::Divider(DividerStyle::Dot));
    }

    #[test]
    fn two_line_pairs_stats() {
        let p = |k: StatKind| Pill { kind: k, label: k.label(), text: "x".into(), warning: false };
        let items = vec![
            BarItem::Pill(p(StatKind::CpuUsage)),
            BarItem::Pill(p(StatKind::CpuTemp)),
            BarItem::Pill(p(StatKind::RamUsed)),
            BarItem::Divider(DividerStyle::Line),
            BarItem::Pill(p(StatKind::Ping)),
        ];
        assert_eq!(
            two_line_columns(&items),
            vec![
                Column::Pills(p(StatKind::CpuUsage), Some(p(StatKind::CpuTemp))),
                Column::Pills(p(StatKind::RamUsed), None),
                Column::Divider(DividerStyle::Line),
                Column::Pills(p(StatKind::Ping), None),
            ]
        );
    }

    #[test]
    fn hover_summary_from_history() {
        let mut h = History::new(60);
        for v in [10.0, 20.0, 30.0] {
            h.push(v);
        }
        assert_eq!(hover_summary(StatKind::CpuUsage, &h), "min 10% · avg 20% · max 30%");
        assert_eq!(hover_summary(StatKind::CpuUsage, &History::new(60)), "no data yet");
    }
}
