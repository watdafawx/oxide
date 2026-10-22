The Appearance page gets a slider for the gap between stats: 0 to 24 pixels. Dragging gives a mouse x in window pixels. Turning that into a gap is maths, so it gets written and tested before any mouse is involved.

**Share of the track.** Where is `x` along the track, from 0 at the left edge to 1 at the right? `(x - left) / width`. The mouse can go past either end while dragging, so clamp it to 0..=1.

**Into the range.** A share of 0.5 on a 0 to 24 slider is 12. In general: `min + share * (max - min)`.

**Snap to the step.** A gap of 12.37 pixels is silly; the slider moves in whole steps. Count steps from `min`, round, multiply back:

```rust
let snapped = min + ((raw - min) / step).round() * step;
```

**Clamp once more.** Rounding near the top could step past `max`; a final clamp keeps the answer in range.

## Your turn

- In `slider_value`, work out the share (clamped), the raw value, and the snapped value.
- Give back the snapped value, clamped to `min..=max`.
- Save.
