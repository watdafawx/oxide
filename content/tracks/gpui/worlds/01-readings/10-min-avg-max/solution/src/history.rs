//! The last minute of readings.

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

    /// Every reading, oldest first.
    pub fn values(&self) -> impl Iterator<Item = f64> + '_ {
        self.values.iter().copied()
    }
}
