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

pub use order::{DividerStyle, Slot, default_order};

/// Arrange page, Alt+Up: swap the slot at `index` with the one before it.
/// Returns false, and changes nothing, when it cannot move.
pub fn move_up(order: &mut [Slot], index: usize) -> bool {
    let _ = (order, index);
    // TODO: index 0 or past the end -> false. Otherwise swap index - 1 and index, then true.
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_up_swaps_with_previous() {
        let mut order = default_order();
        assert!(move_up(&mut order, 1));
        assert_eq!(order[0], Slot::Stat(StatKind::CpuTemp));
        assert_eq!(order[1], Slot::Stat(StatKind::CpuUsage));
    }

    #[test]
    fn first_slot_cannot_move_up() {
        let mut order = default_order();
        assert!(!move_up(&mut order, 0));
        assert!(!move_up(&mut order, 11));
        assert_eq!(order, default_order());
    }
}
