//! The overlay pages (OBS browser sources) get the HUD's state live over a WebSocket.

#![allow(unused_imports)]

use axum::Router;
use axum::extract::State;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::Response;
use axum::routing::get;
use serde::Serialize;
use tokio::sync::broadcast;

use crate::*;

/// What every overlay page shows.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct HudState {
    pub blue: u32,
    pub orange: u32,
    pub time_seconds: u32,
    pub overtime: bool,
    pub last_goal: Option<String>,
}

impl HudState {
    /// Take the score and clock from the game's state.
    pub fn apply(&mut self, update: &UpdateState) {
        for team in &update.game.teams {
            match team.team_num {
                0 => self.blue = team.score,
                _ => self.orange = team.score,
            }
        }
        self.time_seconds = update.game.time_seconds;
        self.overtime = update.game.overtime;
    }
}

/// The overlays' WebSocket: `/ws` sends every update as JSON text.
pub fn overlay_router(updates: broadcast::Sender<String>) -> Router {
    Router::new().route("/ws", get(overlay_socket)).with_state(updates)
}

async fn overlay_socket(ws: WebSocketUpgrade, State(updates): State<broadcast::Sender<String>>) -> Response {
    let rx = updates.subscribe();
    ws.on_upgrade(move |socket| forward(socket, rx))
}

/// Send every update to one overlay until it goes away. An overlay too slow to keep up skips what it
/// missed; nobody waits for it.
pub async fn forward(mut socket: WebSocket, mut updates: broadcast::Receiver<String>) {
    loop {
        match updates.recv().await {
            Ok(text) => {
                if socket.send(Message::Text(text.into())).await.is_err() {
                    return;
                }
            }
            Err(broadcast::error::RecvError::Lagged(_)) => continue,
            Err(broadcast::error::RecvError::Closed) => return,
        }
    }
}
