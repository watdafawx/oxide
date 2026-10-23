//! The Stats API's wire format: JSON objects back to back on a TCP stream, nothing between them.

#![allow(unused_imports)]

use serde_json::Value;

use crate::*;

/// Bytes read so far that do not make a whole frame yet.
#[derive(Debug, Default)]
pub struct FrameBuffer {
    pub pending: Vec<u8>,
}

impl FrameBuffer {
    /// Add what one read gave and take out every whole frame. A half frame waits for the next read.
    pub fn push(&mut self, bytes: &[u8]) -> Vec<Value> {
        self.pending.extend_from_slice(bytes);
        let (frames, used) = split_frames(&self.pending);
        self.pending.drain(..used);
        frames
    }
}
