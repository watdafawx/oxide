The bar and the Settings window are separate views in separate windows. Both need the same `Settings`, and a change in one must reach the other. Passing a copy around would give each its own. GPUI keeps one value for the whole app: a global.

**The word for it.** A **global** is a value the app holds once, keyed by its type. `cx.set_global(Settings::default())` stores it; `cx.global::<Settings>()` reads it from any view, any window. `main.rs` sets it before anything opens.

**Opt in.** A type becomes a global by implementing the marker trait `Global`, which has nothing to write. `settings.rs` now has `impl Global for Settings {}`.

**Read it.** `StatsBar::from_settings` builds a bar from the app's settings: the interval, then `apply_settings` copies the look, order, colours and left-click action onto the bar. It still makes fresh defaults instead of reading the app's:

```rust
let settings = cx.global::<Settings>();
```

**Set before read.** Reading a global that was never set panics. That is a programming mistake, not a user one, so a loud failure is right.

## Your turn

- In `from_settings`, replace the defaults line with `cx.global::<Settings>()`.
- Save.
