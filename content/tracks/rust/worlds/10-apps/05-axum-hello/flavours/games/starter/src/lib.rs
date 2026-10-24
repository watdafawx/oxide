use axum::{Router, routing::get};

/// The handler for GET /status: the plain text `online`.
async fn status() -> &'static str {
    // TODO: return "online"
    todo!()
}

/// The whole API: one route.
pub fn app() -> Router {
    // TODO: replace the fallback with a single route: GET /status handled by `status`
    Router::new().fallback(status)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt; // for `oneshot`

    /// Sends one request straight into the router and returns (status, body text).
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
    async fn status_is_200_online() {
        let (status, body) = send("GET", "/status").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "online");
    }

    #[tokio::test]
    async fn post_to_status_is_405() {
        let (status, _) = send("POST", "/status").await;
        assert_eq!(status, StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn unknown_path_is_404() {
        let (status, _) = send("GET", "/nope").await;
        assert_eq!(status, StatusCode::NOT_FOUND);
    }
}
