//! hud: a Rocket League broadcast HUD server.

pub mod frames;

#[allow(unused_imports)]
pub use frames::*;

#[allow(unused_imports)]
use serde_json::Value;

/// Every whole JSON value at the start of `bytes`, and how many bytes they used. An unfinished last value is left
/// for later; anything that is not JSON at all is thrown away with the rest of the bytes.
pub fn split_frames(bytes: &[u8]) -> (Vec<Value>, usize) {
    let _ = bytes;
    todo!("every whole value, and how many bytes they used")
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

    #[test]
    fn three_frames_from_one_read() {
        let (frames, used) = split_frames(br#"{"Event":"A"}{"Event":"B"} {"Event":"C"}"#);
        assert_eq!(frames.len(), 3);
        assert_eq!(frames[2]["Event"], "C");
        assert_eq!(used, 40);
    }

    #[test]
    fn half_frame_waits() {
        let (values, used) = split_frames(br#"{"Event":"A"}{"Ev"#);
        assert_eq!((values.len(), used), (1, 13), "the half frame is not used yet");
        let mut buffer = FrameBuffer::default();
        let mut got = Vec::new();
        for chunk in wire().chunks(700) {
            got.extend(buffer.push(chunk));
        }
        assert_eq!(got, frames(), "every frame, whole, in order");
        assert!(buffer.pending.is_empty());
    }
}
