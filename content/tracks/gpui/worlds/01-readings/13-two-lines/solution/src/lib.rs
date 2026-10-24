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

#[cfg(test)]
mod tests {
    use super::*;

    fn p(kind: StatKind) -> Pill {
        Pill { kind, label: kind.label(), text: "x".into(), warning: false }
    }

    #[test]
    fn two_line_pairs_stats() {
        let items: Vec<BarItem> =
            [StatKind::CpuUsage, StatKind::CpuTemp, StatKind::GpuUsage, StatKind::GpuTemp].map(|k| BarItem::Pill(p(k))).to_vec();
        assert_eq!(
            two_line_columns(&items),
            vec![
                Column::Pills(p(StatKind::CpuUsage), Some(p(StatKind::CpuTemp))),
                Column::Pills(p(StatKind::GpuUsage), Some(p(StatKind::GpuTemp))),
            ]
        );
    }

    #[test]
    fn a_divider_ends_a_pair() {
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
            ],
            "RAM waits alone before the divider, and Ping alone at the end"
        );
    }
}
