use axum::routing::post;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

/// The body of POST /total-size.
#[derive(Deserialize)]
pub struct SizesRequest {
    pub sizes_mb: Vec<u32>,
}

#[derive(Serialize)]
pub struct Total {
    pub total_mb: u32,
}

/// POST /total-size {"sizes_mb": [120, 30]} -> {"total_mb": 150}
async fn total_size(Json(req): Json<SizesRequest>) -> Json<Total> {
    Json(Total {
        total_mb: req.sizes_mb.iter().sum(),
    })
}

pub fn app() -> Router {
    Router::new().route("/total-size", post(total_size))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt; // for `oneshot`

    /// POSTs a JSON body straight to the router and returns (status, body as JSON or Null).
    async fn post_json(uri: &str, body: &str) -> (StatusCode, serde_json::Value) {
        let req = Request::builder()
            .method("POST")
            .uri(uri)
            .header("content-type", "application/json")
            .body(Body::from(body.to_string()))
            .unwrap();
        let res = app().oneshot(req).await.unwrap();
        let status = res.status();
        let bytes = axum::body::to_bytes(res.into_body(), usize::MAX)
            .await
            .unwrap();
        let json = serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null);
        (status, json)
    }

    #[tokio::test]
    async fn total_size_adds_the_files() {
        let (status, json) = post_json("/total-size", r#"{"sizes_mb": [120, 30]}"#).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["total_mb"], 150);
        let (_, json) = post_json("/total-size", r#"{"sizes_mb": [4, 4, 400]}"#).await;
        assert_eq!(json["total_mb"], 408);
    }

    #[tokio::test]
    async fn a_body_of_the_wrong_shape_is_refused() {
        let (status, _) = post_json("/total-size", r#"{"size": 5}"#).await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    }
}
