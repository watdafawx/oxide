//! hud: a Rocket League broadcast HUD server.

pub mod frames;
pub mod event;
pub mod feed;
pub mod dedup;

#[allow(unused_imports)]
pub use frames::*;
#[allow(unused_imports)]
pub use event::*;
#[allow(unused_imports)]
pub use feed::*;
#[allow(unused_imports)]
pub use dedup::*;

#[allow(unused_imports)]
use std::net::{SocketAddr, UdpSocket};
#[allow(unused_imports)]
use std::time::Duration;
#[allow(unused_imports)]
use axum::extract::ws::{Message, WebSocket};
#[allow(unused_imports)]
use image::{RgbaImage, imageops};
#[allow(unused_imports)]
use serde_json::Value;
#[allow(unused_imports)]
use tauri::{AppHandle, Emitter, Runtime};
#[allow(unused_imports)]
use tokio::io::AsyncReadExt;
#[allow(unused_imports)]
use tokio::net::TcpStream;
#[allow(unused_imports)]
use tokio::sync::{broadcast, mpsc};

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

#[cfg(test)]
#[allow(unused_imports, dead_code, unused_variables, unused_mut)]
mod tests {
    use super::*;
    use std::time::Duration;

    const MATCH: &str = include_str!("../fixtures/match.jsonl");

    /// The recorded match's frames, without their times.
    fn frames() -> Vec<serde_json::Value> {
        MATCH.lines().map(|line| serde_json::from_str::<serde_json::Value>(line).unwrap()["frame"].clone()).collect()
    }

    /// The recorded match's frames, back to back, as the Stats API sends them.
    fn wire() -> Vec<u8> {
        frames().iter().flat_map(|f| serde_json::to_vec(f).unwrap()).collect()
    }

    /// Every event in the recorded match, with its time.
    fn events() -> Vec<(Duration, GameEvent)> {
        MATCH
            .lines()
            .filter_map(|line| {
                let v: serde_json::Value = serde_json::from_str(line).unwrap();
                normalize(v["frame"].clone()).map(|e| (Duration::from_millis(v["t_ms"].as_u64().unwrap()), e))
            })
            .collect()
    }

    fn goal(scorer: &str, team_num: u8, goal_time: u32) -> Goal {
        Goal { scorer: PlayerRef { name: scorer.into(), team_num }, assister: None, goal_time }
    }

    fn secs(s: f64) -> Duration {
        Duration::from_secs_f64(s)
    }

    #[test]
    fn double_goal_fires_once() {
        let mut dedup = GoalDedup::default();
        assert!(!dedup.is_repeat(&goal("Wren", 0, 280), secs(20.0)));
        assert!(dedup.is_repeat(&goal("Wren", 0, 280), secs(20.3)), "the same goal again 0.3 s later");
        let mut dedup = GoalDedup::default();
        let mut repeats = 0;
        for (t, event) in events() {
            if let GameEvent::GoalScored(g) = &event
                && dedup.is_repeat(g, t)
            {
                repeats += 1;
            }
        }
        assert_eq!(repeats, 1, "one goal in the match is sent twice");
    }

    #[test]
    fn two_real_goals_both_fire() {
        let mut dedup = GoalDedup::default();
        assert!(!dedup.is_repeat(&goal("Moss", 1, 55), secs(245.0)));
        assert!(!dedup.is_repeat(&goal("Moss", 1, 54), secs(246.0)), "same scorer, a second later on the clock: another goal");
        assert!(!dedup.is_repeat(&goal("Pike", 1, 54), secs(246.5)), "another scorer");
    }
}
