use axum::extract::Path;
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;

#[derive(Serialize)]
pub struct TierInfo {
    pub mb: u32,
    pub tier: u32,
}

/// Tier 1 under 100 MB, one more tier every 100 MB.
pub fn tier_for(mb: u32) -> u32 {
    mb / 100 + 1
}

/// GET /status (last quest).
async fn status() -> &'static str {
    "online"
}

/// GET /tier/{mb} -> {"mb": 250, "tier": 3}
async fn tier(Path(mb): Path<u32>) -> Json<TierInfo> {
    Json(TierInfo {
        mb,
        tier: tier_for(mb),
    })
}

pub fn app() -> Router {
    Router::new()
        .route("/status", get(status))
        .route("/tier/{mb}", get(tier))
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
    async fn tier_route_answers_in_json() {
        let (status, json) = get_json("/tier/250").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(json, serde_json::json!({"mb": 250, "tier": 3}));
        let (_, json) = get_json("/tier/0").await;
        assert_eq!(json["tier"], 1);
    }

    #[tokio::test]
    async fn a_path_that_is_not_a_number_is_400() {
        let (status, _) = get_json("/tier/lots").await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }
}
