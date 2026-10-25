//! hud: a Rocket League broadcast HUD server.

pub mod frames;
pub mod event;
pub mod feed;
pub mod dedup;
pub mod grace;
pub mod moments;

#[allow(unused_imports)]
pub use frames::*;
#[allow(unused_imports)]
pub use event::*;
#[allow(unused_imports)]
pub use feed::*;
#[allow(unused_imports)]
pub use dedup::*;
#[allow(unused_imports)]
pub use grace::*;
#[allow(unused_imports)]
pub use moments::*;

#[allow(unused_imports)]
use std::time::Duration;
#[allow(unused_imports)]
use serde_json::Value;

impl MomentTracker {
    /// The moments one event makes, in order.
    pub fn step(&mut self, event: &GameEvent) -> Vec<Moment> {
        let mut moments = Vec::new();
        if let GameEvent::UpdateState(state) = event {
            let game = &state.game;
            let _ = game;
            // TODO: overtime fires once, when game.overtime turns true: compare it with self.overtime, then remember it.
            // TODO: 30 seconds left fires once as the clock gets to 30 or below, never in overtime.
            //       A clock above 30 arms it again (self.thirty_armed).
        }
        moments
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

    fn update(time_seconds: u32, overtime: bool) -> GameEvent {
        GameEvent::UpdateState(UpdateState { game: Game { time_seconds, overtime, teams: vec![] } })
    }

    #[test]
    fn thirty_seconds_fires_once() {
        let mut tracker = MomentTracker::default();
        let mut fired = Vec::new();
        for (time, overtime) in [(32, false), (31, false), (30, false), (29, false), (0, false), (1, true), (40, true), (20, true)] {
            fired.extend(tracker.step(&update(time, overtime)));
        }
        assert_eq!(fired, vec![Moment::ThirtySecondsLeft, Moment::Overtime]);
    }
}
