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
pub mod cuts;
pub mod blocking;

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
pub use cuts::*;
#[allow(unused_imports)]
pub use blocking::*;

#[allow(unused_imports)]
use std::net::{SocketAddr, UdpSocket};
#[allow(unused_imports)]
use std::time::Duration;
#[allow(unused_imports)]
use axum::extract::ws::{Message, WebSocket};
#[allow(unused_imports)]
use image::{RgbaImage, imageops};
#[allow(unused_imports)]
use serde_json::Value;
#[allow(unused_imports)]
use tauri::{AppHandle, Emitter, Runtime};
#[allow(unused_imports)]
use tokio::io::AsyncReadExt;
#[allow(unused_imports)]
use tokio::net::TcpStream;
#[allow(unused_imports)]
use tokio::sync::{broadcast, mpsc};

/// Run blocking or CPU-heavy work on tokio's blocking threads and wait for it, without holding up other tasks.
pub async fn off_the_runtime<T: Send + 'static>(work: impl FnOnce() -> T + Send + 'static) -> T {
    tokio::task::spawn_blocking(work).await.expect("the work panicked")
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

    #[tokio::test]
    async fn feed_keeps_flowing_while_cutting() {
        use std::sync::atomic::{AtomicU32, Ordering};
        let ticks = std::sync::Arc::new(AtomicU32::new(0));
        let counter = ticks.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_millis(10)).await;
                counter.fetch_add(1, Ordering::Relaxed);
            }
        });
        let done = off_the_runtime(|| {
            std::thread::sleep(Duration::from_millis(300));
            "cut 3 photos"
        })
        .await;
        assert_eq!(done, "cut 3 photos");
        let ticks = ticks.load(Ordering::Relaxed);
        assert!(ticks >= 5, "the feed task kept running during the work: {ticks} ticks");
    }

    #[tokio::test]
    async fn cut_folder_from_async() {
        let input = scratch("async-in");
        let output = scratch("async-out");
        photo().save(input.join("wren.png")).unwrap();
        let (from, to) = (input.clone(), output.clone());
        assert_eq!(off_the_runtime(move || cut_folder(&from, &to, 32)).await, Ok(1));
        assert!(output.join("wren.png").exists());
    }
}
