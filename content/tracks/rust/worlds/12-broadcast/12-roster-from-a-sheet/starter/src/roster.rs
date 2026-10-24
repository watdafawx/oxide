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
