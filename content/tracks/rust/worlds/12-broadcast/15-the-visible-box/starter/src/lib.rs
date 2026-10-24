//! hud: a Rocket League broadcast HUD server.

pub mod frames;
pub mod event;
pub mod feed;
pub mod dedup;
pub mod grace;
pub mod moments;
pub mod triggers;
pub mod osc;
pub mod overlay;
pub mod roster;
pub mod names;

#[allow(unused_imports)]
pub use frames::*;
#[allow(unused_imports)]
pub use event::*;
#[allow(unused_imports)]
pub use feed::*;
#[allow(unused_imports)]
pub use dedup::*;
#[allow(unused_imports)]
pub use grace::*;
#[allow(unused_imports)]
pub use moments::*;
#[allow(unused_imports)]
pub use triggers::*;
#[allow(unused_imports)]
pub use osc::*;
#[allow(unused_imports)]
pub use overlay::*;
#[allow(unused_imports)]
pub use roster::*;
#[allow(unused_imports)]
pub use names::*;

#[allow(unused_imports)]
use std::time::Duration;
#[allow(unused_imports)]
use image::RgbaImage;
#[allow(unused_imports)]
use serde_json::Value;

/// The box around every pixel that is not fully transparent: (left, top, right, bottom), both edges included.
/// `None` for a fully transparent image.
pub fn visible_box(img: &RgbaImage) -> Option<(u32, u32, u32, u32)> {
    let _ = img;
    // TODO: walk every pixel; widen the box (left, top, right, bottom) around each one whose alpha is above 0.
    // No such pixel at all: None.
    None
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

    /// Every event in the recorded match, with its time.
    fn events() -> Vec<(Duration, GameEvent)> {
        MATCH
            .lines()
            .filter_map(|line| {
                let v: serde_json::Value = serde_json::from_str(line).unwrap();
                normalize(v["frame"].clone()).map(|e| (Duration::from_millis(v["t_ms"].as_u64().unwrap()), e))
            })
            .collect()
    }

    fn goal(scorer: &str, team_num: u8, goal_time: u32) -> Goal {
        Goal { scorer: PlayerRef { name: scorer.into(), team_num }, assister: None, goal_time }
    }

    fn secs(s: f64) -> Duration {
        Duration::from_secs_f64(s)
    }

    fn triggers() -> Vec<Trigger> {
        load_triggers(include_str!("../fixtures/triggers.json")).unwrap()
    }

    /// A 100 x 60 transparent picture with a solid 30 x 20 block at (20, 10).
    fn photo() -> RgbaImage {
        let mut img = RgbaImage::new(100, 60);
        for x in 20..50 {
            for y in 10..30 {
                img.put_pixel(x, y, image::Rgba([200, 40, 40, 255]));
            }
        }
        img
    }

    /// A fresh, empty folder for one test.
    fn scratch(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("hud-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn box_holds_every_visible_pixel() {
        assert_eq!(visible_box(&photo()), Some((20, 10, 49, 29)), "the 30 x 20 block at (20, 10), both edges included");
        let mut dot = RgbaImage::new(8, 8);
        dot.put_pixel(3, 5, image::Rgba([0, 0, 0, 1]));
        assert_eq!(visible_box(&dot), Some((3, 5, 3, 5)), "one barely visible pixel is a 1 x 1 box");
    }

    #[test]
    fn empty_photo_has_no_box() {
        assert_eq!(visible_box(&RgbaImage::new(8, 8)), None, "a fully transparent image has no box");
        assert!(visible_box(&photo()).is_some());
    }
}
