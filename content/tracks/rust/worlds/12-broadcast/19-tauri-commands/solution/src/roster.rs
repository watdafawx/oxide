//! The roster, from a sheet exported as CSV. Columns are found by their header, never by position:
//! producers add, move and rename the other columns all the time.

#![allow(unused_imports)]

use crate::*;

/// The headers this roster needs: the "sheet type".
pub const NICKNAME: &str = "Nickname";
pub const TEAM: &str = "Team Name";
pub const PHOTO: &str = "Photo Front";

#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub nickname: String,
    pub team: String,
    pub photo: String,
}

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
