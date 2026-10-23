//! hud: a Rocket League broadcast HUD server.

pub mod frames;
pub mod event;
pub mod feed;

#[allow(unused_imports)]
pub use frames::*;
#[allow(unused_imports)]
pub use event::*;
#[allow(unused_imports)]
pub use feed::*;

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

/// Read one connection until it closes, sending every event on. `Err` when nobody listens any more.
pub async fn read_frames(mut stream: TcpStream, events: &mpsc::Sender<GameEvent>) -> Result<(), mpsc::error::SendError<GameEvent>> {
    let mut frames = FrameBuffer::default();
    let mut chunk = [0u8; 4096];
    loop {
        let read = match stream.read(&mut chunk).await {
            Ok(0) | Err(_) => return Ok(()),
            Ok(n) => n,
        };
        for frame in frames.push(&chunk[..read]) {
            if let Some(event) = normalize(frame) {
                events.send(event).await?;
            }
        }
    }
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

    #[tokio::test]
    async fn frames_arrive_in_order() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            use tokio::io::AsyncWriteExt;
            let (mut socket, _) = listener.accept().await.unwrap();
            for chunk in wire().chunks(333) {
                socket.write_all(chunk).await.unwrap();
            }
        });
        let (tx, mut rx) = mpsc::channel(1024);
        let stream = TcpStream::connect(addr).await.unwrap();
        tokio::spawn(async move { read_frames(stream, &tx).await });
        let mut got = Vec::new();
        while let Ok(Some(event)) = tokio::time::timeout(Duration::from_secs(5), rx.recv()).await {
            got.push(event);
        }
        let want: Vec<GameEvent> = events().into_iter().map(|(_, e)| e).collect();
        assert_eq!(got.len(), want.len());
        assert_eq!(got, want);
    }

    #[tokio::test]
    async fn reconnects_after_drop() {
        assert_eq!(backoff(6, Duration::from_secs(1)), Duration::from_secs(10));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            use tokio::io::AsyncWriteExt;
            for name in ["First", "Second"] {
                let (mut socket, _) = listener.accept().await.unwrap();
                socket.write_all(format!(r#"{{"Event":"{name}"}}"#).as_bytes()).await.unwrap();
            }
        });
        let (tx, mut rx) = mpsc::channel(8);
        tokio::spawn(run_feed(addr, Duration::from_millis(10), tx));
        let wait = Duration::from_secs(5);
        let first = tokio::time::timeout(wait, rx.recv()).await.expect("a first event");
        let second = tokio::time::timeout(wait, rx.recv()).await.expect("an event after reconnecting");
        assert_eq!(first, Some(GameEvent::Other("First".into())));
        assert_eq!(second, Some(GameEvent::Other("Second".into())));
    }
}
