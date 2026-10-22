Kinetik does things when you touch its bar: a left click opens Task Manager, hovering shows a graph, a right click opens a menu. It all starts with knowing which pill was clicked. The bar keeps that in `ui`, a `BarUi` with everything this world needs.

**One function, every pill.** `view.rs` now hands every pill to `interactive_pill` in `lib.rs` before drawing it. Whatever you add there, every pill gets, in one-line and two-line mode alike.

**An id first.** A click is a mouse-down and a mouse-up on the same element, so GPUI has to remember the element between frames. `.id(..)` gives it a stable name and turns the `Div` into a `Stateful<Div>`, which can take clicks. The kind's name is unique per pill:

```rust
.id(SharedString::from(format!("pill-{kind:?}")))
```

**Then the listener.** `.on_click(..)` takes a closure. `cx.listener(..)` wraps it so it gets `this`, the bar itself, to change: set `this.ui.clicked` to `Some(kind)` and notify.

**`move` for the kind.** The closure runs later, after `interactive_pill` has returned. `move` copies `kind` into it.

## Your turn

- In `interactive_pill`, replace the TODO with `.id(..)` from the kind.
- Add `.on_click(cx.listener(move |this, _, _, cx| { .. }))` that sets `this.ui.clicked` and notifies.
- Save.
