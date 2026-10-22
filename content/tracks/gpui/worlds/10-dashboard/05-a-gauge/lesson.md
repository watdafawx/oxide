Kinetik's dashboard cards have gauges: a dial that fills as usage rises. A half circle from left to right, like a speedometer, drawn as many short line segments along its curve. Each segment needs points on the circle.

**Angles on a circle.** On a circle, an angle of π radians points left, π/2 points up and 0 points right. Share 0 should be left and 1 right, so the angle is `π × (1 − share)`.

**From angle to point.** A point on a circle is the centre plus `radius × cos(angle)` across and `radius × sin(angle)` up. On screen, y grows downward, so "up" is minus:

```rust
point(center.x + radius * angle.cos(), center.y - radius * angle.sin())
```

**Drawn in pieces.** The card's `gauge` asks for 33 points from 0 to the share and joins them with `line_to`: a grey track first, the coloured fill over it.

**Only for shares.** Percentages and temperatures get a gauge (`bar_share` from the widget); bytes and speeds do not.

## Your turn

- In `gauge_point`, work out the angle from the share (clamped to 0..=1).
- Give back the point on the circle: x plus the cosine part, y minus the sine part.
- Save. Press Run: CPU and GPU cards get dials.
