Commands are the pages asking Rust. A hat-trick happens on Rust's side, and the pages should hear about it at once instead of asking every second. That is an event.

**Push, not poll.** `emit` sends a named event with a JSON payload to every page. The React side subscribes once with `listen("moment", ..)`.

**A small payload.** The pages need the cue (to pick the graphic) and the detail (the player or team). `MomentPayload` derives `Serialize`:

```rust
app.emit("moment", MomentPayload { cue: moment.cue(), detail: moment.detail() })
```

**Any runtime.** `publish_moment` takes an `AppHandle<R>` for any `Runtime`, so the same code runs in the real app and on the mock runtime in tests.

**Rust listens too.** `listen_any` subscribes from Rust, which is how the test checks what was sent.

## Your turn

- In `publish_moment`, emit a `moment` event with the moment's cue and detail.
- Save.
