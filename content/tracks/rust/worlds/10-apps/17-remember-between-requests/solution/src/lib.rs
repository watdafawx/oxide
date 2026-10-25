use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

/// What every handler shares: one XP counter.
#[derive(Clone, Default)]
pub struct AppState {
    pub xp: Arc<AtomicU32>,
}

#[derive(Deserialize)]
pub struct Award {
    pub amount: u32,
}

#[derive(Serialize)]
pub struct XpTotal {
    pub xp: u32,
}

/// POST /xp {"amount": 50} -> 201 {"xp": <new total>}
async fn award(
    State(state): State<AppState>,
    Json(req): Json<Award>,
) -> (StatusCode, Json<XpTotal>) {
    let xp = state.xp.fetch_add(req.amount, Ordering::SeqCst) + req.amount;
    (StatusCode::CREATED, Json(XpTotal { xp }))
}

/// GET /xp -> {"xp": <total>}
async fn current(State(state): State<AppState>) -> Json<XpTotal> {
    Json(XpTotal {
        xp: state.xp.load(Ordering::SeqCst),
    })
}

pub fn app() -> Router {
    Router::new()
        .route("/xp", get(current).post(award))
        .with_state(AppState::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt; // for `oneshot`

    async fn send(app: &Router, method: &str, body: &str) -> (StatusCode, serde_json::Value) {
        let req = Request::builder()
            .method(method)
            .uri("/xp")
            .header("content-type", "application/json")
            .body(Body::from(body.to_string()))
            .unwrap();
        // A Router is cheap to clone, and its clones share the same state.
        let res = app.clone().oneshot(req).await.unwrap();
        let status = res.status();
        let bytes = axum::body::to_bytes(res.into_body(), usize::MAX)
            .await
            .unwrap();
        let json = serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null);
        (status, json)
    }

    #[tokio::test]
    async fn award_answers_201_and_the_new_total() {
        let app = app();
        let (status, json) = send(&app, "POST", r#"{"amount": 50}"#).await;
        assert_eq!(status, StatusCode::CREATED);
        assert_eq!(json["xp"], 50);
        let (_, json) = send(&app, "POST", r#"{"amount": 30}"#).await;
        assert_eq!(json["xp"], 80);
    }

    #[tokio::test]
    async fn the_total_survives_between_requests() {
        let app = app();
        send(&app, "POST", r#"{"amount": 40}"#).await;
        send(&app, "POST", r#"{"amount": 60}"#).await;
        let (status, json) = send(&app, "GET", "").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["xp"], 100);
    }
}
