//! hud: a Rocket League broadcast HUD server.

pub mod frames;
pub mod event;
pub mod feed;
pub mod dedup;
pub mod grace;
pub mod moments;
pub mod triggers;

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
pub use triggers::*;

#[allow(unused_imports)]
use std::time::Duration;
#[allow(unused_imports)]
use serde_json::Value;

/// Every trigger in a `triggers.json`. A mistake comes back with its line number.
pub fn load_triggers(text: &str) -> Result<Vec<Trigger>, String> {
    let _ = text;
    Ok(Vec::new()) // TODO: read the list; an error says its line
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

    fn triggers() -> Vec<Trigger> {
        load_triggers(include_str!("../fixtures/triggers.json")).unwrap()
    }

    #[test]
    fn goal_maps_to_companion_press() {
        let triggers = triggers();
        assert_eq!(triggers.len(), 5);
        assert_eq!(triggers[0], Trigger { on: "goal".into(), output: Output::Companion { page: 1, row: 0, column: 3 } });
        assert_eq!(triggers[1].output, Output::Osc { address: "/notch/hattrick".into(), args: vec![OscArg::Int(1)] });
        assert_eq!(triggers[3].output.label(), "http http://127.0.0.1:9010/trigger/overtime");
    }

    #[test]
    fn bad_output_kind_says_where() {
        let err = load_triggers(include_str!("../fixtures/triggers-bad.json")).unwrap_err();
        assert!(err.contains("line 3"), "{err}");
        assert!(err.contains("dmx"), "{err}");
    }
}
