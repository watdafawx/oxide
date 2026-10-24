//! Turning numbers into the text the bar shows.

/// A percentage with no decimals: 12.6 shows "13%".
pub fn format_percent(value: f64) -> String {
    format!("{value:.0}%")
}

/// Bytes in the biggest unit that keeps the number at 1 or more: 1536 shows "1.5 KB".
pub fn format_bytes(bytes: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut value = bytes as f64;
    let mut unit = 0;
    while value >= 1024.0 && unit < units.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{bytes} B")
    } else {
        format!("{value:.1} {}", units[unit])
    }
}

/// A network speed: "↓ 3.4 MB/s".
pub fn format_speed(arrow: &str, bytes_per_sec: f64) -> String {
    format!("{arrow} {}/s", format_bytes(bytes_per_sec as u64))
}
