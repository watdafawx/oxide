The HUD graphics are web pages in OBS browser sources: the scorebug, the goal stinger, the team logos. rl-hud pushes the state to them over socket.io; here it is a plain axum WebSocket. Each open page gets its own task running `forward`.

**Many listeners.** Every overlay needs every update. A `tokio::sync::broadcast` channel gives each subscriber its own copy of each message; `overlay_socket` (already written) subscribes one receiver per connection. An `mpsc` channel could not do this: it hands each message to one receiver only.

**Never wait for laggards.** A broadcast channel holds a fixed number of messages. A receiver that falls behind gets `Err(RecvError::Lagged(n))`, with `n` the number it missed, and jumps to the oldest message still kept. The sender never waits, so one slow page never stalls the feed.

**Loop on the receiver.** `updates.recv().await` waits for the next message and gives back `Ok(text)`, `Lagged`, or `Closed` once every sender is gone. A `loop` with a `match` handles all three; the same shape writing updates into a log instead of a socket:

```rust
loop {
    match updates.recv().await {
        Ok(text) => log.push(text),
        Err(broadcast::error::RecvError::Lagged(missed)) => log.push(format!("missed {missed}")),
        Err(broadcast::error::RecvError::Closed) => return log,
    }
}
```

**Lagged is not fatal.** For a scorebug, only the latest state matters. Skip what was missed with `continue`, which jumps straight to the next turn of the loop.

**Text into a message.** A WebSocket sends `Message`s. `Message::Text` takes axum's own text type, not a `String`, so write `Message::Text(text.into())`: `.into()` converts the `String` into whatever the variant needs, the way `.into()` turned `&str` into a `Box<dyn Error>` in the Errors world.

**Leaving is normal.** `socket.send(message).await` gives an `Err` when the page has closed. Return; the task ends and its receiver is dropped.

## Your turn

- In `forward`, loop on `updates.recv().await` and send each text to the socket as `Message::Text`.
- On `Lagged`, continue; on `Closed` or a failed send, return. Save.
