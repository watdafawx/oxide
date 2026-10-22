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

pub use history::History;

/// Minimum, average and maximum of a history: the hover card's three numbers.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Summary {
    pub min: f64,
    pub avg: f64,
    pub max: f64,
}

impl History {
    /// None while there are no readings yet.
    pub fn summary(&self) -> Option<Summary> {
        if self.is_empty() {
            return None;
        }
        let mut min = f64::MAX;
        let mut max = f64::MIN;
        let mut total = 0.0;
        for v in self.values() {
            min = min.min(v);
            max = max.max(v);
            total += v;
        }
        Some(Summary { min, avg: total / self.len() as f64, max })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_avg_max_of_10_20_30() {
        let mut h = History::new(60);
        for v in [20.0, 10.0, 30.0] {
            h.push(v);
        }
        assert_eq!(h.summary(), Some(Summary { min: 10.0, avg: 20.0, max: 30.0 }));
    }

    #[test]
    fn summary_of_empty_is_none() {
        assert_eq!(History::new(60).summary(), None);
    }
}
