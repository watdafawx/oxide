On Kinetik's Arrange page every stat can show an icon, a text label, or nothing at all in front of its value. People who know their bar by heart switch labels off to save taskbar space. `StatsBar` now has a `label_mode`, and every `PillView` gets `show_label`.

**Breaking the chain.** An `if` cannot sit in the middle of a builder chain. You would have to stop the chain in a `let`, write the `if`, add the label in one branch, and start chaining again: clumsy, for one optional child.

**The word for it.** `.when(condition, |el| ..)` is a link that runs the closure only when the condition is true. The closure gets the element so far and gives back the element with more added. When the condition is false, the element passes through unchanged. The bar's two-line mode uses it to shrink the text:

```rust
.when(self.two_lines, |el| el.text_xs())
```

**The closure's job.** `el` is the pill so far, handed over by value like every link in the chain. The closure hands it back with something added, and `when` carries on the chain from there.

**Gone, not hidden.** With labels off, the label box is not drawn at all: it takes no space, and the test finds no `label-CpuUsage`. Making it invisible would leave a gap.

## Your turn

- In `PillView::render`, put the label's `.child(..)` inside a `.when(..)` on `self.show_label`.
- Save. Then try `label_mode: LabelMode::Off` in `demo` and press Run.
