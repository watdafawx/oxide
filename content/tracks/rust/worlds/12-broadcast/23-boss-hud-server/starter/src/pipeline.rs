//! The whole server as one step function: an event in, the actions it causes out.
//! No sockets inside, so a recorded match replays exactly the same every time.

#![allow(unused_imports)]

use std::time::Duration;

use crate::*;

/// One thing the server does: a cue at a time, with the outputs its triggers send.
#[derive(Debug, Clone, PartialEq)]
pub struct Action {
    pub at: Duration,
    pub cue: &'static str,
    pub detail: String,
    pub outputs: Vec<Output>,
}

impl Action {
    /// One log line: milliseconds, cue, detail, then where it went.
    pub fn line(&self) -> String {
        let mut line = format!("{:>7} {}", self.at.as_millis(), self.cue);
        if !self.detail.is_empty() {
            line.push(' ');
            line.push_str(&self.detail);
        }
        if !self.outputs.is_empty() {
            line.push_str(" -> ");
            line.push_str(&self.outputs.iter().map(Output::label).collect::<Vec<_>>().join(", "));
        }
        line
    }
}

pub struct Pipeline {
    pub hud: HudState,
    pub grace: Grace,
    pub dedup: GoalDedup,
    pub moments: MomentTracker,
    pub triggers: Vec<Trigger>,
}

impl Pipeline {
    pub fn new(triggers: Vec<Trigger>) -> Self {
        Pipeline { hud: HudState::default(), grace: Grace::default(), dedup: GoalDedup::default(), moments: MomentTracker::default(), triggers }
    }

    /// The feed (re)connected at `now`.
    pub fn connected(&mut self, now: Duration) {
        self.grace.connected(now);
    }

    /// The outputs of every trigger on `cue`.
    pub fn outputs_for(&self, cue: &str) -> Vec<Output> {
        self.triggers.iter().filter(|t| t.on == cue).map(|t| t.output.clone()).collect()
    }
}
