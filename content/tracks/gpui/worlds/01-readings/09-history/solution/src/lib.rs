//! Kinetik's stat model.

pub mod format;
pub mod limits;
pub mod reading;
pub mod stat;

pub use format::{format_bytes, format_percent, format_speed};
pub use limits::{Limit, Limits, WarnWhen, is_warning};
pub use reading::Reading;
pub use stat::{ALL, StatKind, Unit};

use std::collections::VecDeque;

/// The last readings of one stat, oldest first. Feeds the hover graph.
#[derive(Debug, Clone)]
pub struct History {
    values: VecDeque<f64>,
    cap: usize,
}

impl History {
    /// An empty history that keeps at most `cap` readings.
    pub fn new(cap: usize) -> Self {
        History { values: VecDeque::new(), cap }
    }

    /// Add the newest reading; the oldest goes once there are more than `cap`.
    pub fn push(&mut self, value: f64) {
        self.values.push_back(value);
        if self.values.len() > self.cap {
            self.values.pop_front();
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn oldest(&self) -> Option<f64> {
        self.values.front().copied()
    }

    pub fn newest(&self) -> Option<f64> {
        self.values.back().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn history_keeps_last_60() {
        let mut h = History::new(60);
        for second in 0..100 {
            h.push(second as f64);
        }
        assert_eq!(h.len(), 60);
        assert_eq!(h.oldest(), Some(40.0));
        assert_eq!(h.newest(), Some(99.0));
    }
}
