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

/// Every whole JSON value at the start of `bytes`, and how many bytes they used. An unfinished last value is left
/// for later; anything that is not JSON at all is thrown away with the rest of the bytes.
pub fn split_frames(bytes: &[u8]) -> (Vec<Value>, usize) {
    let mut stream = serde_json::Deserializer::from_slice(bytes).into_iter::<Value>();
    let mut frames = Vec::new();
    loop {
        let start = stream.byte_offset();
        match stream.next() {
            Some(Ok(frame)) => frames.push(frame),
            Some(Err(e)) if e.is_eof() => return (frames, start),
            Some(Err(_)) => return (frames, bytes.len()),
            None => return (frames, stream.byte_offset()),
        }
    }
}
