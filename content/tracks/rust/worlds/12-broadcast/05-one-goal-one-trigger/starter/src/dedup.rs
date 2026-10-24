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
