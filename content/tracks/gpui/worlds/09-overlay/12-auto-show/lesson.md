With "Show automatically in games" on, Kinetik's overlay appears when a game is in front, hides when you leave it, and closes with the game. That is a small state machine: what it does depends on what it did before.

**Remember the game.** `AutoShow` keeps the game's process id and whether the overlay is shown. Each check gets what is in front and a way to ask "is this process still running?".

**Exit first.** If the remembered game is no longer running, close and forget it, whatever is in front now. Checked later, an exited game would look like "something else is in front", and the overlay would only hide and wait for a game that never comes back. `if let Some(game) = self.game && !running(game)` asks both at once.

**Start over.** Forgetting means putting every field back to its start value. `self` here is a `&mut AutoShow`, a reference, and `*self = AutoShow::default();` writes a fresh value into the place it points at, all fields at once. It is the same `*` as `*r += n` in [Follow the arrow](/quest/rust/02-ownership/09-follow-the-arrow).

**Show and hide.** A game in front while hidden shows the overlay and remembers the game; something else in front while shown hides it. A match guard from the live-frames quest says "only if". Here the same shape warns once when the GPU runs hot, and again when it has cooled down:

```rust
match gpu_temp {
    Some(t) if t >= 90.0 && !self.warned => { self.warned = true; Some(Alert::Hot) }
    Some(t) if t < 80.0 && self.warned => { self.warned = false; Some(Alert::Cool) }
    _ => None,
}
```

**Nothing to do.** Every other case, such as the game still in front while shown, is `None`: no flicker, no repeated show.

## Your turn

- In `step`, close and reset when the remembered game is no longer running.
- Then `match front`: show for a game while hidden, hide for something else while shown, `None` otherwise.
- Save.
