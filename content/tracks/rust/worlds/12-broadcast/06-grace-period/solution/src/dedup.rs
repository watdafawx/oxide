//! The Stats API sometimes sends one goal twice. A repeat must not fire the goal graphics again.

#![allow(unused_imports)]

use std::time::Duration;

use crate::*;

/// A second `GoalScored` this soon after the first, same scorer, same game time, is the same goal.
pub const REPEAT_WINDOW: Duration = Duration::from_secs(2);

#[derive(Debug, Default)]
pub struct GoalDedup {
    /// The last goal that fired: scorer, game time, and when it arrived.
    pub last: Option<(String, u32, Duration)>,
}

impl GoalDedup {
    /// Is this goal the one that just fired, sent again? A goal that is not a repeat is remembered.
    pub fn is_repeat(&mut self, goal: &Goal, now: Duration) -> bool {
        if let Some((scorer, time, at)) = &self.last
            && *scorer == goal.scorer.name
            && *time == goal.goal_time
            && now.saturating_sub(*at) < REPEAT_WINDOW
        {
            return true;
        }
        self.last = Some((goal.scorer.name.clone(), goal.goal_time, now));
        false
    }
}
