//! Right after it connects, the game sends a burst of old state. Nothing fires for the first 2 seconds.

#![allow(unused_imports)]

use std::time::Duration;

use crate::*;

pub const GRACE: Duration = Duration::from_secs(2);

#[derive(Debug, Default)]
pub struct Grace {
    /// When the feed last connected, on the feed's clock.
    pub connected_at: Option<Duration>,
}

impl Grace {
    pub fn connected(&mut self, now: Duration) {
        self.connected_at = Some(now);
    }
}
