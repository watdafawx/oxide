//! How long the PC has been on.

/// How long the PC has been on: "3d 4h", "4h 12m" under a day, "12m" under an hour.
pub fn format_uptime(seconds: u64) -> String {
    let days = seconds / 86_400;
    let hours = seconds % 86_400 / 3600;
    let minutes = seconds % 3600 / 60;
    if days > 0 {
        format!("{days}d {hours}h")
    } else if hours > 0 {
        format!("{hours}h {minutes}m")
    } else {
        format!("{minutes}m")
    }
}
