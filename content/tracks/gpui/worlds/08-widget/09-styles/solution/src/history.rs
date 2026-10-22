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
