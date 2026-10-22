//! Turning numbers into the text the bar shows.

/// A percentage with no decimals: 12.6 shows "13%".
pub fn format_percent(value: f64) -> String {
    format!("{value:.0}%")
}
