Commands are the pages asking Rust. A hat-trick happens on Rust's side, and the pages should hear about it at once instead of asking every second. That is a Tauri **event**: a named message Rust pushes to every page.

**Push, not poll.** `app.emit(name, payload)` sends an event to every page; the payload goes over as JSON. The React side subscribes once with `listen("moment", ..)` and runs its code whenever one arrives. Asking again and again instead is called **polling**, and it is both late and wasteful.

**A small payload.** The pages need the cue (to pick the stinger) and the detail (the player or the team). `MomentPayload` (already written) holds just those two and derives `Serialize`, so any `Serialize` struct works as a payload.

**One call, one event.** Telling the pages the score changed, with a different payload struct, looks like this:

```rust
app.emit("score", ScorePayload { blue: hud.blue, orange: hud.orange })
```

**What it gives back.** `emit` returns a `tauri::Result<()>`: an error if, for example, the payload cannot be turned into JSON. No page listening is not an error; the event simply reaches nobody. `publish_moment` returns the same type, so the call can be its last line.

**Any runtime.** `publish_moment` takes an `AppHandle<R>` for any `R: Runtime`, so the same code runs in the real app and on the mock runtime in tests. `Listener::listen_any` subscribes from Rust, which is how the test checks what was sent.

## Your turn

- In `publish_moment`, emit a `"moment"` event with a `MomentPayload` built from the moment's `cue()` and `detail()`.
- Save.
