With "Show automatically in games" on, Kinetik's overlay appears when a game is in front, hides when you leave it, and closes with the game. That is a small state machine: what it does depends on what it did before.

**Remember the game.** `AutoShow` keeps the game's process id and whether the overlay is shown. Each check gets what is in front and a way to ask "is this process still running?".

**Exit first.** If the remembered game is no longer running, close and forget it, whatever is in front now. Checked later, an exited game would look like "something else is in front" and only hide.

**Show and hide.** A game in front while hidden shows the overlay and remembers the game; something else in front while shown hides it. Guards on the arms say "only if":

```rust
Front::Game(pid) if !self.shown => {
```

**Nothing to do.** Every other case, such as the game still in front, is `None`: no flicker, no repeated show.

## Your turn

- In `step`, close and reset when the remembered game is no longer running.
- Then `match front`: show for a game while hidden, hide for something else while shown, `None` otherwise.
- Save.
