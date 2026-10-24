Point at a stat in Kinetik and it lights up, then shows its hover card. Two separate things happen there: the pill looks different, and the bar remembers which stat you are pointing at, so the card knows what to show.

**Just the look.** `.hover(|style| ..)` changes the style while the mouse is over the element. Its closure gets the style and gives it back changed, for example with a faint white background, `style.bg(rgba(0xffffff14))`; `rgba` is `rgb` with two more digits for see-through. It needs no state and no listener; GPUI handles it.

**The state.** `.on_hover(cx.listener(..))` calls its listener with `true` when the mouse arrives and `false` when it leaves. That bool is this listener's event, so the closure names it: `hovered: &bool`.

**Through the reference.** `hovered` is a `&bool`, a reference to the bool, not the bool itself. `if` wants a real `bool`, so read the value behind the reference with `*hovered`, as in [Rust world 02](/quest/rust/02-ownership/09-follow-the-arrow).

**The obvious version.** Set the stat on `true`, clear it on `false`:

```rust
.on_hover(cx.listener(move |this, hovered: &bool, _, cx| {
    this.ui.hovered = if *hovered { Some(kind) } else { None };
    cx.notify();
}))
```

**Leave only your own.** That version has a race. Moving quickly from CPU to RAM can deliver "RAM: true" before "CPU: false", and then CPU's late goodbye wipes out RAM. On `false`, clear `hovered` only if it still names this pill's kind.

**Same for two lines.** `view.rs` sends two-line pills through `interactive_pill` too, so hover works in both layouts with no extra code.

## Your turn

- In `interactive_pill`, add `.hover(..)` with the faint background.
- Add `.on_hover(..)`: on `true` set `this.ui.hovered = Some(kind)`; on `false` clear it only if it is still `Some(kind)`; then notify.
- Save. Press Run and move over the bar.
