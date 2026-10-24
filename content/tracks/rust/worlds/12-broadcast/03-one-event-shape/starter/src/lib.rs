//! hud: a Rocket League broadcast HUD server.

pub mod frames;
pub mod event;

#[allow(unused_imports)]
pub use frames::*;
#[allow(unused_imports)]
pub use event::*;

#[allow(unused_imports)]
use std::time::Duration;
#[allow(unused_imports)]
use serde_json::Value;

/// Any frame, Stats API or SOS, as a `GameEvent`. `None` for a frame that is not an event, or a known event whose data is wrong.
pub fn normalize(frame: Value) -> Option<GameEvent> {
    let _ = frame;
    // TODO: read the frame as an Envelope (None if it is not one), then match on stats_api_name of its event:
    // the four known names read their data into the right struct, any other name becomes GameEvent::Other.
    todo!("read the Envelope, then match the name")
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
    fn goal_scored_parses() {
        let frame = serde_json::json!({"Event": "GoalScored", "Data": {"Scorer": {"Name": "Wren", "TeamNum": 0}, "GoalTime": 280}});
        assert_eq!(normalize(frame), Some(GameEvent::GoalScored(goal("Wren", 0, 280))));
        let goals = events().iter().filter(|(_, e)| matches!(e, GameEvent::GoalScored(_))).count();
        assert_eq!(goals, 9, "seven goals, one sent twice, one stale goal from before the match");
    }

    #[test]
    fn sos_and_stats_api_agree() {
        let data = serde_json::json!({"Attacker": {"Name": "Fen", "TeamNum": 0}, "Victim": {"Name": "Moss", "TeamNum": 1}});
        let stats = normalize(serde_json::json!({"Event": "Demolish", "Data": data.clone()}));
        let sos = normalize(serde_json::json!({"event": "game:demolish", "data": data}));
        assert!(matches!(stats, Some(GameEvent::Demolish(_))));
        assert_eq!(sos, stats);
    }

    #[test]
    fn unknown_event_kept() {
        let replay = serde_json::json!({"Event": "ReplayCreated", "Data": {"Length": 9}});
        assert_eq!(normalize(replay), Some(GameEvent::Other("ReplayCreated".into())));
        assert_eq!(normalize(serde_json::json!({"Event": "GoalScored", "Data": {"Oops": 1}})), None, "a known event with broken data");
        assert_eq!(normalize(serde_json::json!([1, 2])), None, "not an event at all");
    }
}
