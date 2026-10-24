//! hud: a Rocket League broadcast HUD server.

pub mod frames;
pub mod event;
pub mod feed;
pub mod dedup;
pub mod grace;

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
use std::time::Duration;
#[allow(unused_imports)]
use serde_json::Value;

impl Grace {
    /// May anything fire now? Not before a connection, and not in its first 2 seconds.
    pub fn allows(&self, now: Duration) -> bool {
        self.connected_at.is_some_and(|at| now.saturating_sub(at) >= GRACE)
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
    fn boot_burst_ignored() {
        let mut grace = Grace::default();
        assert!(!grace.allows(secs(5.0)), "not connected yet");
        grace.connected(secs(10.0));
        assert!(!grace.allows(secs(10.4)));
        assert!(!grace.allows(secs(11.9)));
        assert!(grace.allows(secs(12.0)));
        grace.connected(secs(60.0));
        assert!(!grace.allows(secs(61.0)), "a reconnect starts the wait again");
    }
}
