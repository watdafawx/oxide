You built an axum JSON API in the Apps world. The dashboard's web side is the same: a router with two routes, the page and `/api/stats`. What is new is living next to GPUI.

**Two runtimes.** GPUI runs its own executor on the main thread; axum needs tokio. They do not mix. The web server gets its own thread with its own tokio runtime, and the two sides share one `Arc<Mutex<WebSnapshot>>`: the app writes, the server reads.

**The routes.** `router` maps `/` to a small HTML page (`dashboard.html`, which fetches the stats every second) and `/api/stats` to your handler. `State` hands each request the shared state.

**The handler.** Lock the snapshot, clone it out so the lock is held as briefly as possible, and reply with it as JSON:

```rust
Json(snapshot).into_response()
```

**Tested without a network.** The test sends a request straight into the router with `tower`'s `oneshot`. No port, no firewall prompt.

## Your turn

- In `stats`, lock `state.snapshot`, clone it, and give back `Json(snapshot).into_response()`.
- Save.
