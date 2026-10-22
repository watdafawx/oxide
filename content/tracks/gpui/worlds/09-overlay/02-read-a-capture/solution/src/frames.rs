//! Frame times: what FPS, lows and stutter are made of.

#[allow(unused_imports)]
use std::collections::VecDeque;

/// One presented frame: when it reached the screen, how long since the one before, and
/// whether the game drew it or frame generation made it up.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Frame {
    pub time: f64,
    pub ms: f64,
    pub generated: bool,
}
