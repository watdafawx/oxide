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
            // TODO: a GoalScored arm: count the scorer's goals in self.goals; the third one (exactly 3) is a hat-trick.
            // TODO: a Demolish arm: add 1 to the attacker's team in self.demos and push a Demo with that team's new count.
            _ => {}
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

    #[test]
    fn hat_trick_on_third_goal() {
        let mut tracker = MomentTracker::default();
        let mut fired = Vec::new();
        for (who, team) in [("Wren", 0), ("Pike", 1), ("Wren", 0), ("Wren", 0), ("Wren", 0)] {
            fired.extend(tracker.step(&GameEvent::GoalScored(goal(who, team, 100))));
        }
        assert_eq!(fired, vec![Moment::HatTrick("Wren".into())], "on the third goal, not again on the fourth");
    }

    #[test]
    fn demos_counted_per_team() {
        let demo = |attacker: &str, team: u8, victim: &str| {
            GameEvent::Demolish(Demo { attacker: PlayerRef { name: attacker.into(), team_num: team }, victim: PlayerRef { name: victim.into(), team_num: 1 - team } })
        };
        let mut tracker = MomentTracker::default();
        let mut fired = Vec::new();
        for event in [demo("Wren", 0, "Pike"), demo("Teal", 1, "Ash"), demo("Fen", 0, "Moss")] {
            fired.extend(tracker.step(&event));
        }
        assert_eq!(fired, vec![Moment::Demo { team: 0, count: 1 }, Moment::Demo { team: 1, count: 1 }, Moment::Demo { team: 0, count: 2 }]);
        assert_eq!(fired[2].detail(), "blue 2");
    }
}
