A ticking bar is touched from several places: the window draws it, a timer updates it, later a settings window changes it. Rust allows one `&mut` at a time, so nobody can just hold the bar. GPUI holds it for everyone and lends it out on request.

**The word for it.** An **entity** is state that GPUI owns. `cx.new(|cx| ...)` builds the state, hands it to GPUI, and gives you back an `Entity<StatsBar>`: a handle, cheap to clone, that names the bar without owning it.

**Ask to look.** To read the bar you ask the app: `bar.read_with(cx, |bar, _| ...)` lends a `&StatsBar` for the length of the closure. It is the same idea as `Rc<RefCell<..>>` from Smart pointers, with GPUI as the owner.

**Make one.** `new_bar` builds a bar entity from a snapshot. `main.rs` opens the window with it, and the tests read it back:

```rust
cx.new(|_| StatsBar::new(snapshot))
```

**Why it matters.** The window and the timer both hold an `Entity<StatsBar>`. Neither owns the bar, so neither blocks the other.

## Your turn

- In `new_bar`, replace `todo!()` with the `cx.new(..)` call.
- Save. Press Run: same bar, now owned by GPUI.
