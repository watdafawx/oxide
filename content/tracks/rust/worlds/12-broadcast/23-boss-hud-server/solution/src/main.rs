use std::time::{Duration, Instant};

use broadcast_23_boss_hud_server::*;

/// Replays the recorded match and prints what the server does.
/// `cargo run -- --live` instead connects to Rocket League's Stats API and serves overlays on ws://127.0.0.1:8787/ws.
#[tokio::main]
async fn main() {
    let triggers = load_triggers(include_str!("../fixtures/triggers.json")).expect("fixtures/triggers.json reads");
    if !std::env::args().any(|arg| arg == "--live") {
        for line in replay(include_str!("../fixtures/match.jsonl"), triggers).expect("the recording reads") {
            println!("{line}");
        }
        return;
    }
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], STATS_API_PORT));
    let (events_tx, mut events) = tokio::sync::mpsc::channel(256);
    tokio::spawn(run_feed(addr, Duration::from_secs(1), events_tx));
    let (updates, _) = tokio::sync::broadcast::channel(16);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8787").await.expect("port 8787 is free");
    let router = overlay_router(updates.clone());
    tokio::spawn(async move { axum::serve(listener, router).await });
    println!("Waiting for Rocket League on {addr}. Overlays: ws://127.0.0.1:8787/ws");
    let start = Instant::now();
    let mut pipeline = Pipeline::new(triggers);
    pipeline.connected(Duration::ZERO);
    while let Some(event) = events.recv().await {
        for action in pipeline.step(&event, start.elapsed()) {
            println!("{}", action.line());
        }
        let _ = updates.send(serde_json::to_string(&pipeline.hud).unwrap_or_default());
    }
}
