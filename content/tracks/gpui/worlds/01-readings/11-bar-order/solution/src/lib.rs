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
    fn default_order_starts_with_cpu() {
        let order = default_order();
        assert_eq!(order.first(), Some(&Slot::Stat(StatKind::CpuUsage)));
        assert_eq!(order.get(1), Some(&Slot::Stat(StatKind::CpuTemp)));
    }

    #[test]
    fn default_order_has_three_line_dividers() {
        let order = default_order();
        assert_eq!(order.len(), 11);
        let lines = order.iter().filter(|s| **s == Slot::Divider(DividerStyle::Line)).count();
        assert_eq!(lines, 3);
        assert_eq!(order.last(), Some(&Slot::Stat(StatKind::Ping)));
    }
}
