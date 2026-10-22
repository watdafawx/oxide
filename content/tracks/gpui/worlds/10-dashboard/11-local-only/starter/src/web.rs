//! The web dashboard: the same numbers, as JSON and a small page, behind a secret key.

#![allow(unused_imports)]

use std::net::{Ipv4Addr, SocketAddr};
use std::sync::{Arc, Mutex};

use axum::Json;
use axum::Router;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::get;
use serde::{Deserialize, Serialize};

use crate::stat::ALL;
use crate::*;

/// One stat as the web page gets it.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WebStat {
    pub kind: StatKind,
    pub label: &'static str,
    pub text: String,
    pub value: Option<f64>,
    pub warning: bool,
}

/// Everything the web dashboard shows, ready to turn into JSON.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct WebSnapshot {
    pub stats: Vec<WebStat>,
}

/// The bar's readings, every stat, for the web page.
pub fn web_snapshot(bar: &StatsBar) -> WebSnapshot {
    let limits = Limits::default();
    let stats = ALL
        .into_iter()
        .map(|kind| {
            let reading = Reading { kind, value: bar.snapshot.get(&kind).copied() };
            WebStat { kind, label: kind.label(), text: reading.text(), value: reading.value, warning: is_warning(&reading, &limits) }
        })
        .collect();
    WebSnapshot { stats }
}

/// Compare two keys in time that does not depend on where they first differ,
/// so nobody can guess a key one character at a time by timing the answers.
pub fn keys_match(given: &str, key: &str) -> bool {
    given.len() == key.len() && given.bytes().zip(key.bytes()).fold(0u8, |diff, (a, b)| diff | (a ^ b)) == 0
}

/// A new random key: 16 bytes from the operating system, as 32 hex characters.
pub fn new_key() -> String {
    let mut bytes = [0u8; 16];
    getrandom::fill(&mut bytes).expect("the OS gives random bytes");
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// What the web server shares with the app: the latest snapshot and the key.
#[derive(Clone)]
pub struct WebState {
    pub snapshot: Arc<Mutex<WebSnapshot>>,
    pub key: String,
}

/// `?key=...` in the link.
#[derive(Debug, Deserialize)]
pub struct KeyParam {
    pub key: Option<String>,
}

/// The dashboard page: a little HTML that fetches /api/stats every second.
pub const PAGE: &str = include_str!("dashboard.html");

/// The web dashboard's routes: the page, and the stats as JSON.
pub fn router(state: WebState) -> Router {
    Router::new().route("/", get(page)).route("/api/stats", get(stats)).with_state(state)
}

async fn page() -> Html<&'static str> {
    Html(PAGE)
}

/// GET /api/stats?key=...: the snapshot as JSON, for the right key only.
async fn stats(State(state): State<WebState>, Query(param): Query<KeyParam>) -> Response {
    if !keys_match(param.key.as_deref().unwrap_or(""), &state.key) {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    let snapshot = state.snapshot.lock().unwrap_or_else(|p| p.into_inner()).clone();
    Json(snapshot).into_response()
}
