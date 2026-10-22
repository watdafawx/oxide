//! Kinetik's stat model.

pub mod format;
pub mod reading;
pub mod stat;

pub use format::format_percent;
pub use reading::Reading;
pub use stat::{ALL, StatKind, Unit};

/// Bytes in the biggest unit that keeps the number at 1 or more: 1536 shows "1.5 KB".
pub fn format_bytes(bytes: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB"];
    let value = bytes as f64;
    let unit = 0;
    // TODO: while value is 1024 or more (and unit < units.len() - 1),
    // divide value by 1024.0 and add 1 to unit. Both need `mut` then.
    if unit == 0 {
        format!("{bytes} B")
    } else {
        format!("{value:.1} {}", units[unit])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bytes_pick_the_right_unit() {
        assert_eq!(format_bytes(1536), "1.5 KB");
        assert_eq!(format_bytes(3_565_158), "3.4 MB");
        assert_eq!(format_bytes(8_589_934_592), "8.0 GB");
    }

    #[test]
    fn small_counts_stay_in_bytes() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(1023), "1023 B");
    }
}
