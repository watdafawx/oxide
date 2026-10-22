On Kinetik's Arrange page every stat can show an icon, a text label, or nothing at all in front of its value. People who know their bar by heart switch labels off to save taskbar space. `StatsBar` now has a `label_mode`, and every `PillView` gets `show_label`.

**Breaking the chain.** Elements are built with one long chain of calls. Breaking it apart for an `if` and stitching it back together is clumsy.

**The word for it.** `.when(condition, |el| ...)` runs the closure only when the condition is true. The closure gets the element so far and gives back the element with more added. When the condition is false, the element passes through unchanged:

```rust
.when(self.show_label, |el| el.child(div().debug_selector(move || format!("label-{kind:?}")).child(label)))
```

**The closure's job.** `el` is the pill so far. The closure hands it back with the label added, and `when` carries on the chain from there.

**Gone, not hidden.** With labels off, the label box is not drawn at all: it takes no space, and the test finds no `label-CpuUsage`. Making it invisible would leave a gap.

## Your turn

- In `PillView::render`, wrap the label's `.child(..)` in `.when(self.show_label, |el| el.child(..))`.
- Save. Then try `label_mode: LabelMode::Off` in `demo` and press Run.
