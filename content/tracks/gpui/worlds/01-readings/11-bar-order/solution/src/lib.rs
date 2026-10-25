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

/// The four divider looks from Kinetik's Appearance page.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DividerStyle {
    Line,
    Dotted,
    Dot,
    Blank,
}

/// One place on the bar: a stat or a divider.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Slot {
    Stat(StatKind),
    Divider(DividerStyle),
}

/// The bar a new Kinetik install shows, left to right.
pub fn default_order() -> Vec<Slot> {
    vec![
        Slot::Stat(StatKind::CpuUsage),
        Slot::Stat(StatKind::CpuTemp),
        Slot::Divider(DividerStyle::Line),
        Slot::Stat(StatKind::GpuUsage),
        Slot::Stat(StatKind::GpuTemp),
        Slot::Divider(DividerStyle::Line),
        Slot::Stat(StatKind::RamUsed),
        Slot::Divider(DividerStyle::Line),
        Slot::Stat(StatKind::NetDown),
        Slot::Stat(StatKind::NetUp),
        Slot::Stat(StatKind::Ping),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_order_has_three_line_dividers() {
        let order = default_order();
        assert_eq!(order.len(), 11, "eight stats and three dividers");
        let lines = order.iter().filter(|s| **s == Slot::Divider(DividerStyle::Line)).count();
        assert_eq!(lines, 3);
    }

    #[test]
    fn dividers_sit_between_the_groups() {
        let line = Slot::Divider(DividerStyle::Line);
        let stat = Slot::Stat;
        assert_eq!(
            default_order(),
            vec![
                stat(StatKind::CpuUsage),
                stat(StatKind::CpuTemp),
                line,
                stat(StatKind::GpuUsage),
                stat(StatKind::GpuTemp),
                line,
                stat(StatKind::RamUsed),
                line,
                stat(StatKind::NetDown),
                stat(StatKind::NetUp),
                stat(StatKind::Ping),
            ],
            "a Line divider after the CPU pair, after the GPU pair and after RAM"
        );
    }
}
