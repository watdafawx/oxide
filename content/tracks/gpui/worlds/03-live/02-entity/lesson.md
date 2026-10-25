A ticking bar is touched from several places: the window draws it, a timer updates it, later a settings window changes it. Rust allows one `&mut` at a time, so nobody can just hold the bar. GPUI holds it for everyone and lends it out on request.

**The word for it.** An **entity** is state that GPUI owns. `cx.new(|cx| ..)` runs the closure to build the state, hands it to GPUI, and gives you back an `Entity<StatsBar>`: a handle, cheap to clone, that names the bar without owning it. The closure's own `cx` is for state that needs the app while it is built; the bar does not, so it writes `|_|`.

**You have seen it.** `main.rs` has called `cx.new(..)` since What a view is. That call is what made `StatsBar` a view: an entity whose type implements `Render`, put in a window.

**Ask to look.** To read the bar you ask the app: `bar.read_with(cx, |bar, _| ..)` lends a `&StatsBar` for the length of the closure. It is the same idea as `Rc<RefCell<..>>` from Smart pointers, with GPUI as the owner.

**Make one.** `new_bar` builds a bar entity from a snapshot. `main.rs` opens the window with it, and the tests read it back. Making an entity for something else, like the settings in a later world, has the same shape:

```rust
cx.new(|_| Settings::default())
```

**Why it matters.** The window and the timer both hold an `Entity<StatsBar>`. Neither owns the bar, so neither blocks the other.

**For the curious.** Why handles and not `Rc<RefCell>` itself? GPUI keeps every entity in one store owned by the app, so it always knows which ones exist, can tell their windows to redraw when they change, and drops one once its last handle is gone.

## Your turn

- In `new_bar`, replace `todo!()` with a `cx.new(..)` that makes the bar from `snapshot`.
- Save. Press Run: same bar, now owned by GPUI.
