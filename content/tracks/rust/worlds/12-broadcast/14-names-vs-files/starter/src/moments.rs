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

impl MomentTracker {
    /// The moments one event makes, in order.
    pub fn step(&mut self, event: &GameEvent) -> Vec<Moment> {
        let mut moments = Vec::new();
        match event {
            GameEvent::UpdateState(state) => {
                let game = &state.game;
                if game.overtime && !self.overtime {
                    moments.push(Moment::Overtime);
                }
                self.overtime = game.overtime;
                if game.time_seconds > 30 {
                    self.thirty_armed = true;
                } else if !game.overtime && self.thirty_armed {
                    self.thirty_armed = false;
                    moments.push(Moment::ThirtySecondsLeft);
                }
            }
            GameEvent::GoalScored(goal) => {
                let goals = self.goals.entry(goal.scorer.name.clone()).or_insert(0);
                *goals += 1;
                if *goals == 3 {
                    moments.push(Moment::HatTrick(goal.scorer.name.clone()));
                }
            }
            GameEvent::Demolish(demo) => {
                let team = demo.attacker.team_num.min(1);
                self.demos[team as usize] += 1;
                moments.push(Moment::Demo { team, count: self.demos[team as usize] });
            }
            _ => {}
        }
        moments
    }
}
