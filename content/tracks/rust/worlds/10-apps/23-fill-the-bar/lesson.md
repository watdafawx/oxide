Under the level badge, Oxide draws a bar that fills as you earn XP: 40 of the 100 XP this level needs is a bar 40% full. egui has the widget; what it needs from you is one number between 0.0 (empty) and 1.0 (full).

**A decimal for screens.** egui measures everything in `f32`, a decimal number like `f64` but half the size, which is plenty for pixels and bars. `as f32` turns a whole number into one, the same `as` you used for `f64` in world 01. Divide after converting, or `40 / 100` on whole numbers gives 0.

**Dividing by zero.** A whole-number division by zero panics, but an `f32` division doesn't: `1.0 / 0.0` is infinity and `0.0 / 0.0` is NaN, "not a number". A bar fed NaN draws garbage, so check for a zero `xp_for_next` first and return `0.0`.

**Keep it in range.** Data can be odd: a level-up that hasn't been applied yet can leave 250 XP into a 100 XP level. `x.clamp(0.0, 1.0)` keeps a number inside the range: below 0 becomes 0.0, above 1 becomes 1.0, anything between stays.

**Add any widget.** `egui::ProgressBar::new(fraction)` builds a bar as a value, and `ui.add(widget)` draws a widget you built. `ui.button(..)` and `ui.label(..)` are short forms of exactly that.

```rust
let fraction = (self.health as f32 / self.max_health as f32).clamp(0.0, 1.0);
ui.label(format!("{} / {} HP", self.health, self.max_health));
ui.add(egui::ProgressBar::new(fraction));
```

**Test the number, not the pixels.** The fraction is a plain function, so the first test checks it exactly, edge cases included. The screen test only checks that the text and a progress bar are there; how the bar looks is egui's job.

## Your turn

- In `xp_fraction`, return 0.0 when `xp_for_next` is 0.
- Otherwise divide the two as `f32` and keep the result between 0.0 and 1.0.
- In `ui`, draw the label `40 / 100 XP` and a `ProgressBar` filled to that fraction.
