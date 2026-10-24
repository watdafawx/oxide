//! The replay check: record a match once, replay it after every change, and diff what the server did.

#![allow(unused_imports)]

use std::time::Duration;

use serde::Deserialize;
use serde_json::Value;

use crate::*;

/// One recorded frame and when it arrived, in milliseconds from the start.
#[derive(Debug, Deserialize)]
pub struct Recorded {
    pub t_ms: u64,
    pub frame: Value,
}

/// Run a recording (JSON lines) through a fresh pipeline on its own clock: every action's line, in order.
pub fn replay(recording: &str, triggers: Vec<Trigger>) -> Result<Vec<String>, String> {
    let mut pipeline = Pipeline::new(triggers);
    pipeline.connected(Duration::ZERO);
    let mut lines = Vec::new();
    for (n, line) in recording.lines().enumerate().filter(|(_, l)| !l.trim().is_empty()) {
        let recorded: Recorded = serde_json::from_str(line).map_err(|e| format!("line {}: {e}", n + 1))?;
        if let Some(event) = normalize(recorded.frame) {
            lines.extend(pipeline.step(&event, Duration::from_millis(recorded.t_ms)).iter().map(Action::line));
        }
    }
    Ok(lines)
}

/// Where two runs differ, line by line: `- old` for the golden file's line, `+ new` for this run's. Empty when identical.
pub fn diff_lines(golden: &[String], now: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    for i in 0..golden.len().max(now.len()) {
        match (golden.get(i), now.get(i)) {
            (Some(old), Some(new)) if old == new => {}
            (old, new) => {
                if let Some(old) = old {
                    out.push(format!("- {old}"));
                }
                if let Some(new) = new {
                    out.push(format!("+ {new}"));
                }
            }
        }
    }
    out
}
