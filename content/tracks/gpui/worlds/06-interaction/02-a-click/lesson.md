It all starts with knowing which pill was clicked. The bar keeps that in `ui`, a `BarUi` (in `ui.rs`) with a field for everything this world needs: `clicked`, `hovered`, `menu_at` and more.

**One function, every pill.** `view.rs` now hands every pill to `interactive_pill` in `lib.rs` before drawing it. Whatever you add there, every pill gets, in one-line and two-line mode alike.

**An id first.** A click is a mouse-down and a mouse-up on the same element, often a frame or two apart. Elements are rebuilt every frame, so GPUI needs a name to know it is the same pill both times. `.id(..)` gives that name and turns the `Div` into a `Stateful<Div>`, an element that keeps state between frames and so can take clicks.

**One name per pill.** The name must differ from pill to pill. `format!("pill-{kind:?}")` gives `pill-CpuUsage`, `pill-RamUsed` and so on. `.id(..)` wants a `SharedString`, GPUI's text type that is cheap to copy; `SharedString::from(..)` makes one from the `String`.

**Then the listener.** `.on_click(cx.listener(..))` takes the four-input closure from the last quest. A click stores the pill's kind and calls `cx.notify()` so the bar redraws. On some other element, a click that clears the field would read:

```rust
div()
    .id("clear-button")
    .on_click(cx.listener(|this, _, _, cx| {
        this.ui.clicked = None;
        cx.notify();
    }))
```

**`move` for the kind.** Your closure uses `kind`, so it needs `move` in front, as the last quest showed. Without it the compiler says the closure may outlive the function that borrows `kind`.

## Your turn

- In `interactive_pill`, give the pill an id made from its kind, in place of `"pill"`.
- Add an `.on_click` listener that stores `Some(kind)` in `this.ui.clicked` and notifies.
- Save.
