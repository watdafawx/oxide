The HUD graphics are web pages in OBS browser sources: scorebug, goal stinger, team logos. rl-hud pushes state to them over socket.io; here it is a plain axum WebSocket.

**Many listeners.** Every overlay needs every update. A `tokio::sync::broadcast` channel gives each subscriber its own copy; `overlay_socket` subscribes one receiver per connection.

**Never wait for laggards.** A broadcast channel holds a fixed number of messages. A receiver that falls behind gets `Lagged(n)` and jumps to the oldest message still kept. The sender never waits, so the feed never stalls.

**Lagged is not fatal.** For a scorebug, only the latest state matters. Skip what was missed and carry on:

```rust
Err(broadcast::error::RecvError::Lagged(_)) => continue,
```

**Leaving is normal.** A send that fails means the page closed. Return; the task ends and its receiver is dropped.

## Your turn

- In `forward`, loop on `updates.recv().await` and send each text to the socket.
- On `Lagged`, continue; on `Closed` or a failed send, return. Save.
