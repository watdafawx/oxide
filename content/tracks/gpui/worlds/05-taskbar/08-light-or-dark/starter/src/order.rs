//! The bar's layout: which stats, in which order, with which dividers.

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

/// Arrange page, Alt+Up: swap the slot at `index` with the one before it.
/// Returns false, and changes nothing, when it cannot move.
pub fn move_up(order: &mut [Slot], index: usize) -> bool {
    if index == 0 || index >= order.len() {
        return false;
    }
    order.swap(index - 1, index);
    true
}
