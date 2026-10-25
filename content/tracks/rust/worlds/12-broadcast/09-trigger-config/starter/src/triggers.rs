//! Triggers: which cue sends what, where. Read from `triggers.json`, so producers change them without a rebuild.

#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

use crate::*;

/// One OSC argument. In JSON: a whole number, a decimal, or a string.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OscArg {
    Int(i32),
    Float(f32),
    Str(String),
}

/// Where a trigger goes: `"kind"` picks the variant.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Output {
    /// An OSC message, for Notch or Resolume.
    Osc {
        address: String,
        #[serde(default)]
        args: Vec<OscArg>,
    },
    /// A Companion (Stream Deck) button press.
    Companion { page: u32, row: u32, column: u32 },
    /// A plain HTTP GET, for a dashboard or a lighting desk.
    Http { url: String },
}

impl Output {
    /// Short words for logs and the replay check.
    pub fn label(&self) -> String {
        match self {
            Output::Osc { address, .. } => format!("osc {address}"),
            Output::Companion { page, row, column } => format!("companion {page}/{row}/{column}"),
            Output::Http { url } => format!("http {url}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Trigger {
    /// A cue: goal, hat_trick, thirty_seconds, overtime, demo, match_ended.
    pub on: String,
    pub output: Output,
}
