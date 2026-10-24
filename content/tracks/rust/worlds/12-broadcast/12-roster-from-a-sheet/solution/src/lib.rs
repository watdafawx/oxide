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
use std::time::Duration;
#[allow(unused_imports)]
use serde_json::Value;

/// Every player in a roster CSV, cells trimmed. Rows without a nickname are skipped.
pub fn read_roster(text: &str) -> Result<Vec<Player>, String> {
    let mut reader = csv::Reader::from_reader(text.as_bytes());
    let headers = reader.headers().map_err(|e| e.to_string())?.clone();
    let column = |name: &str| headers.iter().position(|h| h.trim() == name).ok_or_else(|| format!("missing column \"{name}\""));
    let (nick, team, photo) = (column(NICKNAME)?, column(TEAM)?, column(PHOTO)?);
    let mut players = Vec::new();
    for record in reader.records() {
        let record = record.map_err(|e| e.to_string())?;
        let cell = |i: usize| record.get(i).unwrap_or("").trim().to_string();
        if cell(nick).is_empty() {
            continue;
        }
        players.push(Player { nickname: cell(nick), team: cell(team), photo: cell(photo) });
    }
    Ok(players)
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
    fn columns_mapped_by_header() {
        let roster = read_roster(include_str!("../fixtures/roster.csv")).unwrap();
        assert_eq!(roster.len(), 6, "the blank row is skipped");
        assert_eq!(roster[1], Player { nickname: "Fen".into(), team: "Blue Herons".into(), photo: "fen.png".into() });
    }

    #[test]
    fn missing_column_named_in_error() {
        let err = read_roster("Nickname,Team Name\nWren,Blue Herons\n").unwrap_err();
        assert_eq!(err, "missing column \"Photo Front\"");
    }
}
