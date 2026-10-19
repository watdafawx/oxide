**Why now:** the mod database needs an HTTP API so a UI (or a script) can list and add mods, and `axum` is the Rust way to write one.

**axum** is the standard Rust way to write one, and it is built from pieces you already know: functions and traits.

- A **handler** is an `async fn` that returns anything implementing `IntoResponse`: a `&str`, a `String`, a `StatusCode`, `Json<T>`, a tuple of those.
- A **Router** maps paths and methods to handlers: `Router::new().route("/health", get(health))`.
- Anything the router has no route for is a 404, and a wrong method on a known path is a 405. You get both for free.
- A `Router` is a tower `Service`. In tests you don't start a server: call `app.oneshot(request).await` and get the response back in-process. No port, no network, no flakiness.

```rust
use axum::{Router, routing::get};

async fn version() -> &'static str {
    "0.1.0"
}

fn app() -> Router {
    Router::new().route("/version", get(version))
}
```

To run it for real you would add `tokio::net::TcpListener::bind(..)` and `axum::serve(listener, app())`. The tests here skip that on purpose.

## Your turn

In `src/lib.rs`:

- `health`: return the text `ok`.
- `app`: a `Router` with a single route, `GET /health`.
