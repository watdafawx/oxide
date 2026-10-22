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

/// Read a capture: a header line naming the columns, then one line per frame.
/// Columns are found by name, so their order does not matter.
pub fn parse_capture(text: &str) -> Vec<Frame> {
    let mut lines = text.lines();
    let Some(header) = lines.next() else {
        return Vec::new();
    };
    let names: Vec<&str> = header.split(',').collect();
    let column = |name: &str| names.iter().position(|n| *n == name);
    let (Some(time), Some(ms), Some(kind)) = (column("TimeInSeconds"), column("MsBetweenPresents"), column("FrameType")) else {
        return Vec::new();
    };
    lines
        .filter_map(|line| {
            let cells: Vec<&str> = line.split(',').collect();
            Some(Frame {
                time: cells.get(time)?.parse().ok()?,
                ms: cells.get(ms)?.parse().ok()?,
                generated: *cells.get(kind)? == "Generated",
            })
        })
        .collect()
}

/// Frames per second over a run of frame times: how many frames, over how many seconds.
pub fn average_fps(ms: &[f64]) -> f64 {
    let total: f64 = ms.iter().sum();
    if total == 0.0 { 0.0 } else { ms.len() as f64 / (total / 1000.0) }
}

/// The "1% low" (share 0.01) or "0.1% low" (0.001): the slowest frames, averaged, as frames per second.
pub fn low_fps(ms: &[f64], share: f64) -> f64 {
    if ms.is_empty() {
        return 0.0;
    }
    let mut slowest = ms.to_vec();
    slowest.sort_by(|a, b| b.total_cmp(a));
    let n = ((ms.len() as f64 * share).round() as usize).max(1);
    let average = slowest[..n].iter().sum::<f64>() / n as f64;
    1000.0 / average
}

/// The last ten seconds of frames, for the lows and the graph.
#[derive(Debug, Clone, Default)]
pub struct FrameWindow {
    pub frames: VecDeque<Frame>,
}

/// How far back the lows look.
pub const WINDOW_SECONDS: f64 = 10.0;

impl FrameWindow {
    /// Add the newest frame, then drop every frame older than ten seconds before it.
    pub fn push(&mut self, frame: Frame) {
        self.frames.push_back(frame);
        while self.frames.front().is_some_and(|old| old.time < frame.time - WINDOW_SECONDS) {
            self.frames.pop_front();
        }
    }

    /// The frame times in the window, oldest first.
    pub fn ms(&self) -> Vec<f64> {
        self.frames.iter().map(|f| f.ms).collect()
    }
}

/// Stutter: the frames that took more than twice the median frame time, by position.
pub fn spikes(ms: &[f64]) -> Vec<usize> {
    if ms.is_empty() {
        return Vec::new();
    }
    let mut sorted = ms.to_vec();
    sorted.sort_by(f64::total_cmp);
    let mid = sorted.len() / 2;
    let median = if sorted.len().is_multiple_of(2) { (sorted[mid - 1] + sorted[mid]) / 2.0 } else { sorted[mid] };
    ms.iter().enumerate().filter(|(_, m)| **m > 2.0 * median).map(|(i, _)| i).collect()
}
