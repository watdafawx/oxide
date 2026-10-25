use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

/// What every handler shares: one download counter.
#[derive(Clone, Default)]
pub struct AppState {
    pub downloaded_mb: Arc<AtomicU32>,
}

#[derive(Deserialize)]
pub struct Download {
    pub mb: u32,
}

#[derive(Serialize)]
pub struct Downloads {
    pub downloaded_mb: u32,
}

/// POST /downloads {"mb": 120} -> 201 {"downloaded_mb": <new total>}
async fn record_download(
    State(state): State<AppState>,
    Json(req): Json<Download>,
) -> (StatusCode, Json<Downloads>) {
    let downloaded_mb = state.downloaded_mb.fetch_add(req.mb, Ordering::SeqCst) + req.mb;
    (StatusCode::CREATED, Json(Downloads { downloaded_mb }))
}

/// GET /downloads -> {"downloaded_mb": <total>}
async fn downloads(State(state): State<AppState>) -> Json<Downloads> {
    Json(Downloads {
        downloaded_mb: state.downloaded_mb.load(Ordering::SeqCst),
    })
}

pub fn app() -> Router {
    Router::new()
        .route("/downloads", get(downloads).post(record_download))
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
            .uri("/downloads")
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
    async fn download_answers_201_and_the_new_total() {
        let app = app();
        let (status, json) = send(&app, "POST", r#"{"mb": 120}"#).await;
        assert_eq!(status, StatusCode::CREATED);
        assert_eq!(json["downloaded_mb"], 120);
        let (_, json) = send(&app, "POST", r#"{"mb": 30}"#).await;
        assert_eq!(json["downloaded_mb"], 150);
    }

    #[tokio::test]
    async fn the_total_survives_between_requests() {
        let app = app();
        send(&app, "POST", r#"{"mb": 40}"#).await;
        send(&app, "POST", r#"{"mb": 60}"#).await;
        let (status, json) = send(&app, "GET", "").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["downloaded_mb"], 100);
    }
}
