//! Kinetik's stat model.

pub mod reading;
pub mod stat;

pub use reading::Reading;
pub use stat::{ALL, StatKind, Unit};

/// A percentage with no decimals: 12.6 shows "13%".
pub fn format_percent(value: f64) -> String {
    format!("{value}%") // TODO: a whole number: no digits after the point
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percent_rounds_to_whole() {
        assert_eq!(format_percent(12.6), "13%");
        assert_eq!(format_percent(12.4), "12%");
        assert_eq!(format_percent(100.0), "100%");
    }
}
