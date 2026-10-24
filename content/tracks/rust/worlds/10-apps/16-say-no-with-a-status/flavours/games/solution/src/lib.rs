use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SizesRequest {
    pub sizes_mb: Vec<u32>,
}

#[derive(Serialize)]
pub struct Total {
    pub total_mb: u32,
}

/// The body of an error answer: {"error": "why"}.
#[derive(Serialize)]
pub struct ApiError {
    pub error: String,
}

/// POST /total-size {"sizes_mb": [120, 30]} -> {"total_mb": 150}.
/// An empty list is a 400 with an ApiError.
async fn total_size(
    Json(req): Json<SizesRequest>,
) -> Result<Json<Total>, (StatusCode, Json<ApiError>)> {
    if req.sizes_mb.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError {
                error: "no files to download".into(),
            }),
        ));
    }
    Ok(Json(Total {
        total_mb: req.sizes_mb.iter().sum(),
    }))
}

pub fn app() -> Router {
    Router::new().route("/total-size", post(total_size))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt; // for `oneshot`

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
    async fn an_empty_list_is_400_with_an_error() {
        let (status, json) = post_json("/total-size", r#"{"sizes_mb": []}"#).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert!(json["error"].is_string(), "the body should be an ApiError, got {json}");
    }

    #[tokio::test]
    async fn a_full_list_still_answers_200() {
        let (status, json) = post_json("/total-size", r#"{"sizes_mb": [120, 30]}"#).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["total_mb"], 150);
    }
}
