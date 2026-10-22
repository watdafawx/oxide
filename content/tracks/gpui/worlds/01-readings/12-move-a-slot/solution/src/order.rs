//! The bar's layout.

use crate::stat::StatKind;

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
