Kinetik's CPU and GPU cards have gauges: a half-circle dial from left to right, like a speedometer, that fills as usage rises. The card draws it as many short lines along the curve, and every line end is a point on a circle. This quest finds those points.

**Turns in radians.** Maths measures angles in **radians**, and so do Rust's `sin` and `cos`. A full turn is 2π radians, so half a turn is π (about 3.14) and a quarter turn is π/2. Angles start pointing right and grow anticlockwise: 0 points right, π/2 points up, π points left. `std::f32::consts::PI` is π.

**Share to angle.** The dial starts on the left at share 0 and ends on the right at share 1. Left is π and right is 0, so the angle is `π × (1 − share)`: share 0.5 gives π/2, straight up. Clamp the share to 0..=1 first, so 120 % never swings the needle past the end.

**Angle to point.** A point on a circle is the centre, plus `radius × cos(angle)` across and `radius × sin(angle)` up. Straight up, the cosine is 0 (no move across) and the sine is 1 (a whole radius up):

```rust
let angle = std::f32::consts::FRAC_PI_2; // π/2: straight up
let top = point(center.x + radius * angle.cos(), center.y - radius * angle.sin());
```

**Up is minus.** On screen, y grows downward, the same "up is smaller" from the hover card's graph. That is why the sine part is subtracted.

**Drawn in pieces.** The card's `gauge` asks `gauge_point` for 33 points from 0 to the share and joins them with `line_to`: a grey track first, the coloured fill over it. Only percentages and temperatures get a gauge; bytes and speeds do not.

## Your turn

- In `gauge_point`, clamp the share to 0..=1 and turn it into an angle: π on the left, 0 on the right.
- Give back the point on the circle at that angle: across by the cosine part, up by the sine part.
- Save. Press Run: the CPU and GPU cards get dials.
