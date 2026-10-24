use axum::extract::Path;
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;

#[derive(Serialize)]
pub struct LevelInfo {
    pub xp: u32,
    pub level: u32,
}

/// Level 1 at 0 XP, one more level every 100 XP.
pub fn level_for(xp: u32) -> u32 {
    xp / 100 + 1
}

/// GET /health (last quest).
async fn health() -> &'static str {
    "ok"
}

/// GET /level/{xp} -> {"xp": 250, "level": 3}
async fn level(Path(xp): Path<u32>) -> Json<LevelInfo> {
    Json(LevelInfo {
        xp,
        level: level_for(xp),
    })
}

pub fn app() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/level/{xp}", get(level))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt; // for `oneshot`

    /// Hands one GET straight to the router and returns (status, body as JSON or Null).
    async fn get_json(uri: &str) -> (StatusCode, serde_json::Value) {
        let req = Request::builder().uri(uri).body(Body::empty()).unwrap();
        let res = app().oneshot(req).await.unwrap();
        let status = res.status();
        let bytes = axum::body::to_bytes(res.into_body(), usize::MAX)
            .await
            .unwrap();
        let json = serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null);
        (status, json)
    }

    #[tokio::test]
    async fn level_route_answers_in_json() {
        let (status, json) = get_json("/level/250").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(json, serde_json::json!({"xp": 250, "level": 3}));
        let (_, json) = get_json("/level/0").await;
        assert_eq!(json["level"], 1);
    }

    #[tokio::test]
    async fn a_path_that_is_not_a_number_is_400() {
        let (status, _) = get_json("/level/lots").await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }
}
