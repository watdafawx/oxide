The screen first checks that the engine is running at all, with `GET /health`, and expects the text `ok`. That is the smallest useful server, and **axum**, the most used Rust web framework, builds it from two pieces: a function that answers and a table of routes.

**A handler answers.** A **handler** is the function that answers one route. In axum it is an `async fn`, the kind from world 08 that can wait (for a database, a file) without holding up other requests; axum runs it on tokio for you. Its return value becomes the response.

**Text is a response.** A handler that returns text sends `200 OK` with that text as the body. The type is `&'static str`: a piece of text written into the program itself, so it lives as long as the program does (`'static`, named in world 02). Other return types give other responses, as the next quests show.

**A router routes.** `Router::new().route("/health", get(health))` says "a `GET` on `/health` is answered by `health`". `get(..)` is the method part; `post(..)` and `delete(..)` exist too. Each `.route(..)` adds one line to the table and hands the `Router` back, so they chain.

```rust
use axum::{Router, routing::get};

async fn version() -> &'static str {
    "1.4.0"
}

fn app() -> Router {
    Router::new().route("/version", get(version))
}
```

**Free answers.** You never write the failures. A path with no route gets `404 Not Found`; a known path with the wrong method, like `POST /health`, gets `405 Method Not Allowed`. The router knows its own table, so it can tell the two apart.

**Tests without a network.** The test's `send` helper builds a request and hands it straight to the router, which answers it like a function call (`oneshot` means "answer exactly one request"). No port is opened, so the tests are fast and never flaky. A real `main` would add `tokio::net::TcpListener::bind("127.0.0.1:3000")` and `axum::serve(listener, app())`.

## Your turn

- In `health`, return the text `ok`.
- In `app`, build a `Router` with one route: `GET /health`, answered by `health`.
