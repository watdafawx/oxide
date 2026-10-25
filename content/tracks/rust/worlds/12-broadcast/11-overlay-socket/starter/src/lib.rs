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
use std::time::Duration;
#[allow(unused_imports)]
use axum::extract::ws::{Message, WebSocket};
#[allow(unused_imports)]
use serde_json::Value;
#[allow(unused_imports)]
use tokio::net::TcpStream;
#[allow(unused_imports)]
use tokio::sync::broadcast;

/// Send every update to one overlay until it goes away. An overlay too slow to keep up skips what it
/// missed; nobody waits for it.
pub async fn forward(mut socket: WebSocket, mut updates: broadcast::Receiver<String>) {
    let _ = (socket, updates);
    // TODO: send every update; skip on Lagged; stop when closed or the page leaves
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

    type Client = tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>;

    async fn overlay(capacity: usize) -> (broadcast::Sender<String>, Client) {
        let (updates, _) = broadcast::channel(capacity);
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let router = overlay_router(updates.clone());
        tokio::spawn(async move { axum::serve(listener, router).await.unwrap() });
        let (client, _) = tokio_tungstenite::connect_async(format!("ws://{addr}/ws")).await.unwrap();
        // Wait for the server to subscribe this overlay, but never forever.
        let subscribed = async {
            while updates.receiver_count() == 0 {
                tokio::task::yield_now().await;
            }
        };
        let _ = tokio::time::timeout(Duration::from_secs(3), subscribed).await;
        (updates, client)
    }

    async fn next_text(client: &mut Client) -> Option<String> {
        use futures_util::StreamExt;
        let message = tokio::time::timeout(Duration::from_secs(3), client.next()).await.ok()??.ok()?;
        message.into_text().ok().map(|text| text.to_string())
    }

    #[tokio::test]
    async fn overlay_gets_goal() {
        let (updates, mut client) = overlay(16).await;
        let hud = HudState { blue: 1, last_goal: Some("Wren".into()), ..Default::default() };
        updates.send(serde_json::to_string(&hud).unwrap()).unwrap();
        let text = next_text(&mut client).await.expect("the overlay got the update");
        let got: serde_json::Value = serde_json::from_str(&text).unwrap();
        assert_eq!((got["blue"].as_u64(), got["last_goal"].as_str()), (Some(1), Some("Wren")));
    }

    #[tokio::test]
    async fn lagging_overlay_does_not_block() {
        let (updates, mut client) = overlay(2).await;
        for i in 0..10 {
            updates.send(format!("old {i}")).unwrap();
        }
        updates.send("latest".into()).unwrap();
        let mut seen = Vec::new();
        while let Some(text) = next_text(&mut client).await {
            let last = text == "latest";
            seen.push(text);
            if last {
                break;
            }
        }
        assert_eq!(seen.last().map(String::as_str), Some("latest"), "got: {seen:?}");
        assert!(seen.len() < 11, "what it lagged behind on is skipped: {seen:?}");
    }
}
