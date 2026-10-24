use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default)]
pub struct AppState {
    pub installs: Arc<AtomicU64>,
}

#[derive(Serialize)]
pub struct Welcome {
    pub welcome: String,
}

#[derive(Deserialize)]
pub struct SizesRequest {
    pub sizes_kb: Vec<u64>,
}

#[derive(Serialize)]
pub struct Total {
    pub total_kb: u64,
}

#[derive(Serialize)]
pub struct ApiError {
    pub error: String,
}

#[derive(Serialize)]
pub struct Count {
    pub count: u64,
}

/// GET /welcome/{player} -> {"welcome": "Welcome, <player>!"}
async fn welcome(Path(player): Path<String>) -> Json<Welcome> {
    // TODO: return a Welcome wrapped in Json; its text is "Welcome, <player>!"
    todo!()
}

/// POST /total-size {"sizes_kb": [1, 2]} -> {"total_kb": 3}. An empty list is a 400 with an ApiError.
async fn total_size(
    Json(req): Json<SizesRequest>,
) -> Result<Json<Total>, (StatusCode, Json<ApiError>)> {
    // TODO: empty list -> Err((StatusCode::BAD_REQUEST, Json(ApiError { .. }))), else the sum
    todo!()
}

/// POST /installs -> 201 and the new count.
async fn install(State(state): State<AppState>) -> (StatusCode, Json<Count>) {
    // TODO: add 1 to the shared counter (fetch_add gives the OLD value), then answer
    //       201 Created with the new count as JSON
    todo!()
}

/// GET /installs -> the current count.
async fn installs(State(state): State<AppState>) -> Json<Count> {
    // TODO: read the shared counter and return it as a JSON Count
    todo!()
}

pub fn app() -> Router {
    // TODO: add the three routes (GET /welcome/{player}, POST /total-size, and GET + POST on
    //       /installs), then give the router a fresh AppState
    Router::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt; // for `oneshot`

    async fn send(
        app: &Router,
        method: &str,
        uri: &str,
        body: Option<&str>,
    ) -> (StatusCode, serde_json::Value) {
        let req = Request::builder()
            .method(method)
            .uri(uri)
            .header("content-type", "application/json")
            .body(Body::from(body.unwrap_or("").to_string()))
            .unwrap();
        // Router is cheap to clone, and clones share the same State.
        let res = app.clone().oneshot(req).await.unwrap();
        let status = res.status();
        let bytes = axum::body::to_bytes(res.into_body(), usize::MAX)
            .await
            .unwrap();
        let json = serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null);
        (status, json)
    }

    #[tokio::test]
    async fn welcome_greets_the_player_in_json() {
        let app = app();
        let (status, json) = send(&app, "GET", "/welcome/Steve", None).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["welcome"], "Welcome, Steve!");
    }

    #[tokio::test]
    async fn total_size_adds_the_files() {
        let app = app();
        let (status, json) = send(&app, "POST", "/total-size", Some(r#"{"sizes_kb":[1,2,39]}"#)).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["total_kb"], 42);
    }

    #[tokio::test]
    async fn total_size_of_nothing_is_400() {
        let app = app();
        let (status, json) = send(&app, "POST", "/total-size", Some(r#"{"sizes_kb":[]}"#)).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert!(json["error"].is_string(), "body should be an ApiError");
    }

    #[tokio::test]
    async fn installs_count_across_requests() {
        let app = app();
        let (s1, j1) = send(&app, "POST", "/installs", None).await;
        let (s2, j2) = send(&app, "POST", "/installs", None).await;
        assert_eq!(s1, StatusCode::CREATED);
        assert_eq!(s2, StatusCode::CREATED);
        assert_eq!(j1["count"], 1);
        assert_eq!(j2["count"], 2);
        let (_, j3) = send(&app, "GET", "/installs", None).await;
        assert_eq!(j3["count"], 2);
    }
}
