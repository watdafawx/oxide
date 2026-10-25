The web dashboard needs a server: a browser asks for the stats, and Kinetik answers with the snapshot as JSON. You built an axum JSON API in the Rust track's Apps world; this is the same kind of server, with the snapshot shared from the GPUI app.

**How a browser asks.** A browser opening `http://127.0.0.1:8787/api/stats` sends a **request**: a method, `GET` ("give me"), and a path, `/api/stats`. The server sends back a **response**: a status number, where 200 means OK, and a body, here the JSON. That exchange is **HTTP**, and a program that answers it is a **web server**.

**Routes and handlers.** `router` maps each path to an async function, its **handler**: `/` to `page`, which sends `dashboard.html`, and `/api/stats` to `stats`, which is yours. The page then asks `/api/stats` once a second. If `Router`, `get` or `State` feel hazy, the Apps world's two axum quests cover them.

**Shared with the app.** The app writes a fresh snapshot whenever the bar changes, and the server reads it from another thread. So `WebState` holds it as an `Arc<Mutex<WebSnapshot>>`, the pair from the Rust track's Concurrency world: `Arc` so both sides own it, `Mutex` so only one side touches it at a time. `State(state)` hands each request its own clone of `WebState`.

**Lock, copy, let go.** `lock()` waits for the mutex and gives a guard; while the guard lives, the app cannot write. Clone the snapshot out in the same statement and the guard is dropped at its end, so the lock is held for one copy, not while the reply is sent. A sibling handler that only counts the stats looks like this:

```rust
async fn stat_count(State(state): State<WebState>) -> Response {
    let count = state.snapshot.lock().unwrap().stats.len();
    Json(count).into_response()
}
```

**If a thread panicked.** `lock()` gives a `Result`. When a thread panics while it holds the lock, the mutex is **poisoned**, and `unwrap()` would crash the server too. A snapshot is only numbers to show, so take it anyway: `.unwrap_or_else(|p| p.into_inner())` runs its closure only on an `Err`, and `into_inner` takes the data out of the poison error, as in the Concurrency world's mutex quest.

**One reply type.** The handler returns `Response`, axum's type for any reply. `Json(..)` and a `StatusCode` are different types, and `.into_response()` turns either one into a `Response`. That is what lets the next quest's handler answer 200 or 401 from the same function.

**Tested without a network.** The test sends a request straight into the router with `tower`'s `oneshot`. No port, no firewall prompt, no browser.

## Your turn

- In `stats`, lock `state.snapshot`, taking the data even when the lock is poisoned, and clone it out.
- Reply with the snapshot as JSON, turned into a `Response`. Save.
