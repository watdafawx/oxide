//! hud: a Rocket League broadcast HUD server.

pub mod frames;
pub mod event;
pub mod feed;
pub mod dedup;
pub mod grace;
pub mod moments;
pub mod triggers;
pub mod osc;
pub mod overlay;
pub mod roster;
pub mod names;

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
pub use osc::*;
#[allow(unused_imports)]
pub use overlay::*;
#[allow(unused_imports)]
pub use roster::*;
#[allow(unused_imports)]
pub use names::*;

#[allow(unused_imports)]
use std::time::Duration;
#[allow(unused_imports)]
use serde_json::Value;

/// How many single-letter edits (insert, delete, change) turn `a` into `b`.
pub fn levenshtein(a: &str, b: &str) -> usize {
    let _ = (a, b);
    todo!("the edit distance, one row at a time")
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
    fn missing_and_extra_files() {
        let names = ["Wren", "Fen", "Zoë", "Moss", "Bram"];
        let files = ["wren.png", "FEN.png", "zoe.png", "Mos.png", "old_logo.png"];
        let report = compare_names(&names, &files);
        assert_eq!(report.missing, vec!["Bram".to_string()]);
        assert_eq!(report.extra, vec!["old_logo.png".to_string()]);
    }

    #[test]
    fn renamed_file_suggested() {
        assert_eq!(levenshtein("kitten", "sitting"), 3);
        assert_eq!(levenshtein("", "abc"), 3);
        assert_eq!(levenshtein("zoë", "zoe"), 1);
        let report = compare_names(&["Moss"], &["Mos.png"]);
        assert_eq!(report.renames, vec![("Moss".to_string(), "Mos.png".to_string())]);
    }
}
