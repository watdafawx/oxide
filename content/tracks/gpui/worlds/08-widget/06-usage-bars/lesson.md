Each stat in Kinetik's widget can have a coloured usage bar: 70 % CPU is a bar 70 % full. Numbers are exact; bars are quick to read at a glance from across the room.

**How full.** `bar_share(kind, value)` gives 0 to 1: a percentage over 100, a temperature over 110 °C. Bytes and speeds have no natural maximum, so they get no bar.

**Width as a share.** Pixels would need the track's width, which depends on the widget's size. `relative(share)` is a length that is a share of the parent's width, worked out at layout time:

```rust
.w(relative(share))
```

**Track and fill.** The track is a full-width grey strip; the fill sits inside it, starting at the left, as wide as its share. Both get names, so the test can compare their widths.

**Warning colours later.** Kinetik's bars turn the warning colour past a limit. The fill here is always blue; the same `is_warning` from Readings could pick the colour.

## Your turn

- In `usage_bar`, give the fill `.w(relative(share))`.
- Save. Press Run: bars under CPU, GPU and the temperatures.
