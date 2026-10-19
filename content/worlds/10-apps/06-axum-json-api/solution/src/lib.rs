use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default)]
pub struct AppState {
    pub count: Arc<AtomicU64>,
}

#[derive(Serialize)]
pub struct Greeting {
    pub greeting: String,
}

#[derive(Deserialize)]
pub struct SumRequest {
    pub numbers: Vec<i64>,
}

#[derive(Serialize)]
pub struct Total {
    pub total: i64,
}

#[derive(Serialize)]
pub struct ApiError {
    pub error: String,
}

#[derive(Serialize)]
pub struct Count {
    pub count: u64,
}

/// GET /hello/{name} -> {"greeting": "Hello, <name>!"}
async fn hello(Path(name): Path<String>) -> Json<Greeting> {
    Json(Greeting {
        greeting: format!("Hello, {name}!"),
    })
}

/// POST /sum {"numbers": [1, 2]} -> {"total": 3}. An empty list is a 400 with an ApiError.
async fn sum(Json(req): Json<SumRequest>) -> Result<Json<Total>, (StatusCode, Json<ApiError>)> {
    if req.numbers.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError {
                error: "numbers must not be empty".into(),
            }),
        ));
    }
    Ok(Json(Total {
        total: req.numbers.iter().sum(),
    }))
}

/// POST /bump -> 201 and the new count.
async fn bump(State(state): State<AppState>) -> (StatusCode, Json<Count>) {
    let count = state.count.fetch_add(1, Ordering::SeqCst) + 1;
    (StatusCode::CREATED, Json(Count { count }))
}

/// GET /count -> the current count.
async fn count(State(state): State<AppState>) -> Json<Count> {
    Json(Count {
        count: state.count.load(Ordering::SeqCst),
    })
}

pub fn app() -> Router {
    Router::new()
        .route("/hello/{name}", get(hello))
        .route("/sum", post(sum))
        .route("/bump", post(bump))
        .route("/count", get(count))
        .with_state(AppState::default())
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
    async fn hello_greets_by_name_in_json() {
        let app = app();
        let (status, json) = send(&app, "GET", "/hello/Ferris", None).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["greeting"], "Hello, Ferris!");
    }

    #[tokio::test]
    async fn sum_adds_the_numbers() {
        let app = app();
        let (status, json) = send(&app, "POST", "/sum", Some(r#"{"numbers":[1,2,39]}"#)).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["total"], 42);
    }

    #[tokio::test]
    async fn sum_of_nothing_is_400() {
        let app = app();
        let (status, json) = send(&app, "POST", "/sum", Some(r#"{"numbers":[]}"#)).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert!(json["error"].is_string(), "body should be an ApiError");
    }

    #[tokio::test]
    async fn bump_counts_across_requests() {
        let app = app();
        let (s1, j1) = send(&app, "POST", "/bump", None).await;
        let (s2, j2) = send(&app, "POST", "/bump", None).await;
        assert_eq!(s1, StatusCode::CREATED);
        assert_eq!(s2, StatusCode::CREATED);
        assert_eq!(j1["count"], 1);
        assert_eq!(j2["count"], 2);
        let (_, j3) = send(&app, "GET", "/count", None).await;
        assert_eq!(j3["count"], 2);
    }
}
