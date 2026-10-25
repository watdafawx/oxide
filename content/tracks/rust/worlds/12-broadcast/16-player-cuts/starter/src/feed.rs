//! The live feed: connect to the Stats API, read frames, and connect again when the game goes away.

#![allow(unused_imports)]

use std::net::SocketAddr;
use std::time::Duration;

use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::sync::mpsc;

use crate::*;

/// The port Rocket League's Stats API listens on.
pub const STATS_API_PORT: u16 = 49123;

/// How long to wait before reconnect number `attempt` (0 first): 1, 2, 4, 8, then 10 times `base`.
pub fn backoff(attempt: u32, base: Duration) -> Duration {
    base * 2u32.saturating_pow(attempt).min(10)
}

/// Stay connected to the feed: read until it closes, wait, connect again. Stops when nobody listens.
pub async fn run_feed(addr: SocketAddr, base: Duration, events: mpsc::Sender<GameEvent>) {
    let mut attempt = 0;
    while !events.is_closed() {
        if let Ok(stream) = TcpStream::connect(addr).await {
            attempt = 0;
            if read_frames(stream, &events).await.is_err() {
                return;
            }
        }
        tokio::time::sleep(backoff(attempt, base)).await;
        attempt += 1;
    }
}

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
