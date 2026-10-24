use axum::{Router, routing::get};

/// Answers GET /health with the plain text `ok`.
async fn health() -> &'static str {
    // TODO: hand back the text ok
    todo!()
}

/// The whole API: one route.
pub fn app() -> Router {
    // TODO: a Router where GET /health is answered by `health`
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt; // for `oneshot`

    /// Hands one request straight to the router and returns (status, body text).
    async fn send(method: &str, uri: &str) -> (StatusCode, String) {
        let req = Request::builder()
            .method(method)
            .uri(uri)
            .body(Body::empty())
            .unwrap();
        let res = app().oneshot(req).await.unwrap();
        let status = res.status();
        let bytes = axum::body::to_bytes(res.into_body(), usize::MAX)
            .await
            .unwrap();
        (status, String::from_utf8(bytes.to_vec()).unwrap())
    }

    #[tokio::test]
    async fn health_is_200_ok() {
        let (status, body) = send("GET", "/health").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "ok");
    }

    #[tokio::test]
    async fn post_to_health_is_405() {
        let (status, _) = send("POST", "/health").await;
        assert_eq!(status, StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn unknown_path_is_404() {
        let (status, _) = send("GET", "/nope").await;
        assert_eq!(status, StatusCode::NOT_FOUND);
    }
}
