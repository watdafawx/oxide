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

    #[test]
    fn hover_summary_from_history() {
        let mut cpu = History::new(60);
        for v in [10.0, 20.0, 30.0] {
            cpu.push(v);
        }
        assert_eq!(hover_summary(StatKind::CpuUsage, &cpu), "min 10% · avg 20% · max 30%");
        let mut ram = History::new(60);
        ram.push(8_589_934_592.0);
        assert_eq!(hover_summary(StatKind::RamUsed, &ram), "min 8.0 GB · avg 8.0 GB · max 8.0 GB", "RAM is written in bytes");
    }

    #[test]
    fn empty_history_says_no_data_yet() {
        assert_eq!(hover_summary(StatKind::Ping, &History::new(60)), "no data yet");
    }
}
