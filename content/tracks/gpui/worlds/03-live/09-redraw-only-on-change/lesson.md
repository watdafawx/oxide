Kinetik's README brags that it "only redraws when something on the bar has actually changed". At night the PC idles: CPU at 2 % for minutes, the same text every second. Redrawing an identical bar costs a little power and nothing on screen changes.

**Compare first.** `tick` rebuilds the items every time. Build them into a local `items`, and compare with what the bar already shows. `BarItem` derives `PartialEq`, so `!=` compares whole lists, text and colour included.

**Notify on difference.** Store and notify inside the `if`. Same text, same colours: nothing to do, and GPUI has no reason to draw:

```rust
if items != self.items { self.items = items; cx.notify(); }
```

**Text, not numbers.** The comparison is on what the bar shows. CPU going from 2.1 % to 2.4 % is `2%` both times, so it is not a change. That catches most of the idle seconds.

**Still counted.** `ticks` goes up on every read either way. Only the drawing is skipped.

## Your turn

- In `tick`, replace the last two lines with the comparison: store and notify only when `items` differs.
- Save.
