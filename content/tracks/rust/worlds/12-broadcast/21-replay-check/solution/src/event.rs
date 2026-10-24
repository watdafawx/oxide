//! One shape for every game event, whether it came from the official Stats API or BakkesMod SOS.

#![allow(unused_imports)]

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::*;

/// A player as events name them.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlayerRef {
    pub name: String,
    pub team_num: u8,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Team {
    pub name: String,
    pub team_num: u8,
    pub score: u32,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Game {
    /// Seconds left in regulation; seconds played in overtime.
    pub time_seconds: u32,
    #[serde(rename = "bOvertime")]
    pub overtime: bool,
    pub teams: Vec<Team>,
}

/// The game's state, several times a second.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateState {
    pub game: Game,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Goal {
    pub scorer: PlayerRef,
    #[serde(default)]
    pub assister: Option<PlayerRef>,
    /// The game clock when it went in.
    pub goal_time: u32,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Demo {
    pub attacker: PlayerRef,
    pub victim: PlayerRef,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct MatchEnd {
    pub winner_team_num: u8,
}

/// Every event the HUD reacts to. Anything else keeps its name in `Other`.
#[derive(Debug, Clone, PartialEq)]
pub enum GameEvent {
    UpdateState(UpdateState),
    GoalScored(Goal),
    Demolish(Demo),
    MatchEnded(MatchEnd),
    Other(String),
}

/// Every frame on the wire: an event name and its data. SOS spells the keys in lower case.
#[derive(Debug, Deserialize)]
pub struct Envelope {
    #[serde(rename = "Event", alias = "event")]
    pub event: String,
    #[serde(rename = "Data", alias = "data", default)]
    pub data: Value,
}

/// SOS names for the events the Stats API names in PascalCase.
pub const SOS_NAMES: [(&str, &str); 4] = [
    ("game:update_state", "UpdateState"),
    ("game:goal_scored", "GoalScored"),
    ("game:demolish", "Demolish"),
    ("game:match_ended", "MatchEnded"),
];

/// The Stats API's name for an event: an SOS name is swapped for its Stats API twin, any other name stays as it is.
pub fn stats_api_name(name: &str) -> &str {
    for (sos, stats_api) in SOS_NAMES {
        if sos == name {
            return stats_api;
        }
    }
    name
}

/// "blue" for team 0, "orange" for team 1.
pub fn team_name(team_num: u8) -> &'static str {
    if team_num == 0 { "blue" } else { "orange" }
}

/// Any frame, Stats API or SOS, as a `GameEvent`. `None` for a frame that is not an event, or a known event whose data is wrong.
pub fn normalize(frame: Value) -> Option<GameEvent> {
    let Envelope { event, data } = serde_json::from_value(frame).ok()?;
    let name = stats_api_name(&event);
    let parsed = match name {
        "UpdateState" => GameEvent::UpdateState(serde_json::from_value(data).ok()?),
        "GoalScored" => GameEvent::GoalScored(serde_json::from_value(data).ok()?),
        "Demolish" => GameEvent::Demolish(serde_json::from_value(data).ok()?),
        "MatchEnded" => GameEvent::MatchEnded(serde_json::from_value(data).ok()?),
        other => GameEvent::Other(other.to_string()),
    };
    Some(parsed)
}
