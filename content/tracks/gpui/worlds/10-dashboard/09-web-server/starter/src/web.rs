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
