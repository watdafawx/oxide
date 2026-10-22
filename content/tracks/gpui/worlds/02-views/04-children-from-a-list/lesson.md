The model from Readings is now part of this project, and `StatsBar::demo()` fills `pills` from it: eight stats in Kinetik's default order, written and coloured by the rules you made. The bar still draws only the first.

**One element per pill.** `pill_element` already turns one `Pill` into a box: label, value and colour. The bar needs one of those for every pill, and the number of pills changes with the order.

**Many at once.** `.child(..)` adds one element. `.children(..)` takes an iterator and adds every element it gives, in order.

**Map, then hand over.** `self.pills.iter()` walks the pills, and `.map(pill_element)` turns each into an element. The `.children` call collects them, so no `collect` is needed:

```rust
.children(self.pills.iter().map(pill_element))
```

**Names from the kind.** Each pill's box is named after its kind, like `pill-CpuTemp`. That is how the test finds all eight.

## Your turn

- In `render`, replace `.child(pill_element(&self.pills[0]))` with the `.children(..)` line.
- Save. Press Run: the whole default bar, with the hot CPU temperature in red.
