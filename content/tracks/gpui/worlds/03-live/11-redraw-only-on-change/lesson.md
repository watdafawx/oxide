Kinetik's README brags that it "only redraws when something on the bar has actually changed". At night the PC idles: CPU at 2 % for minutes, the same text every second. Redrawing an identical bar costs a little power and nothing on screen changes.

**Compare first.** `tick` rebuilds the items every time. Build them into a local `items`, and compare with what the bar already shows. `BarItem` derives `PartialEq`, so `!=` compares whole lists, text and colour included.

**Notify on difference.** Store and notify inside the `if`. Same text, same colours: nothing to do, and GPUI has no reason to draw. A window title that only redraws when it really changes:

```rust
if title != self.title { self.title = title; cx.notify(); }
```

**Text, not numbers.** The comparison is on what the bar shows. CPU going from 2.1 % to 2.4 % is `2%` both times, so it is not a change. That catches most of the idle seconds.

**Still counted.** `ticks` goes up on every read either way. Only the drawing is skipped.

## Your turn

- In `tick`, put the last two lines inside an `if`, so the new `items` are stored and notified only when they differ from `self.items`.
- Save.
