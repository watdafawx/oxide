use axum::routing::post;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

/// The body of POST /xp-total.
#[derive(Deserialize)]
pub struct XpRequest {
    pub amounts: Vec<u32>,
}

#[derive(Serialize)]
pub struct Total {
    pub total: u32,
}

/// POST /xp-total {"amounts": [50, 30]} -> {"total": 80}
async fn xp_total() -> Json<Total> {
    // TODO: read the JSON body as an XpRequest parameter, add up its amounts,
    //       and answer with a Total as JSON
    todo!()
}

pub fn app() -> Router {
    Router::new().route("/xp-total", post(xp_total))
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
    async fn xp_total_adds_the_amounts() {
        let (status, json) = post_json("/xp-total", r#"{"amounts": [50, 30]}"#).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["total"], 80);
        let (_, json) = post_json("/xp-total", r#"{"amounts": [40, 40, 300]}"#).await;
        assert_eq!(json["total"], 380);
    }

    #[tokio::test]
    async fn a_body_of_the_wrong_shape_is_refused() {
        let (status, _) = post_json("/xp-total", r#"{"amount": 5}"#).await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    }
}
