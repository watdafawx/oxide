Each stat in Kinetik's widget can have a coloured usage bar: 70 % CPU is a bar 70 % full. Numbers are exact; bars are quick to read at a glance from across the room.

**How full.** `bar_share(kind, value)` gives a number from 0 to 1: a percentage over 100, a temperature over 110 °C. Bytes and speeds have no natural maximum, so they get no bar and `bar_share` gives `None`.

**Track and fill.** `usage_bar` draws a grey strip, the track, with a blue strip, the fill, inside it, starting at the left. Each has a `debug_selector` name, so the test can measure both widths and compare them.

**Pixels would need maths.** `.w(px(150.))` is a fixed width. To make the fill 70 % of the track in pixels you would need the track's width, and that depends on how wide the widget is, which you only know once GPUI has laid it out.

**A share of the parent.** `relative(share)` is a length that means "this share of the parent": GPUI works out the pixels at layout time, whatever the widget's size. `.w_full()` and `.h_full()`, already on the track and the fill, are the same idea at 100 %. Here is a box half as tall as its parent:

```rust
div().w_full().h(relative(0.5))
```

**For the curious.** Kinetik's bars turn the warning colour past a limit. The fill here is always blue; the same `is_warning` from the Readings world could pick the colour.

## Your turn

- In `usage_bar`, replace the fill's `px(0.)` width with a width that is `share` of the track.
- Save. Press Run: bars appear under CPU, GPU and the temperatures.
