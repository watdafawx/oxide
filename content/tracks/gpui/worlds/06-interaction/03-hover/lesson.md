Point at a stat in Kinetik and it lights up, then shows its hover card. Two separate things happen there: the pill looks different, and the bar remembers which stat you are pointing at, so the card knows what to show.

**Just the look.** `.hover(|style| ..)` changes the style while the mouse is over the element: here a faint white background, `rgba(0xffffff14)`. It needs no state and no listener; GPUI handles it.

**The state.** `.on_hover(..)` calls its listener with `true` when the mouse arrives and `false` when it leaves:

```rust
.on_hover(cx.listener(move |this, hovered: &bool, _, cx| {
```

**Leave only your own.** Moving quickly from CPU to RAM can deliver "RAM: true" before "CPU: false". Clear `hovered` only if it still names this pill's kind, so RAM is not wiped out by CPU's late goodbye.

**Same for two lines.** `view.rs` sends two-line pills through `interactive_pill` too, so hover works in both layouts with no extra code.

## Your turn

- In `interactive_pill`, add `.hover(..)` with the faint background.
- Add `.on_hover(..)`: on `true` set `this.ui.hovered = Some(kind)`, on `false` clear it only if it is still `Some(kind)`; then notify.
- Save. Press Run and move over the bar.
