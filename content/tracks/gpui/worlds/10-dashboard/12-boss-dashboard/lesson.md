The dashboard's pieces are written: card numbers, columns, text size, gauges, themes, the layout editor, the JSON snapshot, the server's routes, the key, the bind address. This boss draws the window from scratch and starts the web server for real.

**The window.** `render` reads the dashboard settings and the window's width, works out columns, card width and theme, then borrows the bar once and builds a `Card` per visible stat. Settings first, then the bar: `cx.global` and `self.bar.read(cx)` both borrow `cx`, so take what you need from the settings before borrowing the bar.

**The server.** `serve_dashboard` starts a thread, makes a tokio runtime in it, binds the address, and serves the router until the app exits. Errors come back through the thread's result:

```rust
axum::serve(listener, router(state)).await
```

**Wired up.** `main.rs` makes a key on first start, prints the link, starts the server, and copies `web_snapshot` into the shared state every time the bar changes, with `cx.observe_self`.

**Safe by default.** The server binds `127.0.0.1` unless sharing is on, and every API request needs the key: the two rules from the last quests, now live.

## Your turn

- Write `DashboardView::render`: the first four objectives follow it.
- Write `serve_dashboard`.
- Press Run, then open the printed link in a browser.
