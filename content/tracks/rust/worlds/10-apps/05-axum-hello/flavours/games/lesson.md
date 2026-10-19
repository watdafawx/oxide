**Why now:** the mod database should be reachable by other tools: a web UI, a launcher, a script. That means an HTTP API. **axum** is the web framework: you describe routes, each pointing at an `async fn` handler.

```rust
use axum::{routing::get, Router};

async fn status() -> &'static str {
    "online"                               // becomes: 200 OK, body "online"
}

fn app() -> Router {
    Router::new().route("/status", get(status))
}
```

- A **handler** is an `async fn`. Its return value becomes the HTTP response.
- `route(path, get(handler))` binds a path and a method. Other methods on that path get **405**, unknown paths **404**, without any code from you.
- Tests need no network: they call the `Router` directly with `tower::ServiceExt::oneshot(request)` and read the response. Fast and deterministic.
- `app()` returns the `Router`. A real `main` would hand it to a server; tests hand it requests.

## Your turn

In `src/lib.rs`:

- `status`: the handler returning `"online"`.
- `app`: a router with one route, `GET /status`.
