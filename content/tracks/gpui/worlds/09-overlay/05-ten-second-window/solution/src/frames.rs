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
