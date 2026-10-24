//! Moments: what the game feed does not say outright, worked out from its events.

#![allow(unused_imports)]

use std::collections::HashMap;

use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Moment {
    ThirtySecondsLeft,
    Overtime,
    HatTrick(String),
    /// A demolition, and how many that team has made this match.
    Demo { team: u8, count: u32 },
}

impl Moment {
    /// The name triggers use for this moment.
    pub fn cue(&self) -> &'static str {
        match self {
            Moment::ThirtySecondsLeft => "thirty_seconds",
            Moment::Overtime => "overtime",
            Moment::HatTrick(_) => "hat_trick",
            Moment::Demo { .. } => "demo",
        }
    }

    pub fn detail(&self) -> String {
        match self {
            Moment::HatTrick(player) => player.clone(),
            Moment::Demo { team, count } => format!("{} {count}", team_name(*team)),
            _ => String::new(),
        }
    }
}

#[derive(Debug)]
pub struct MomentTracker {
    /// True until "30 seconds left" fires; armed again when the clock is back above 30.
    pub thirty_armed: bool,
    pub overtime: bool,
    pub goals: HashMap<String, u32>,
    /// Demolitions made by team 0 and team 1.
    pub demos: [u32; 2],
}

impl Default for MomentTracker {
    fn default() -> Self {
        MomentTracker { thirty_armed: true, overtime: false, goals: HashMap::new(), demos: [0, 0] }
    }
}
