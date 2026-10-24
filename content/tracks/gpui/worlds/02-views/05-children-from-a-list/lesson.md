The model from Readings is now part of this project, and `StatsBar::demo()` fills `pills` from it: eight stats in Kinetik's default order, written and coloured by the rules you made. The bar still draws only the first.

**One element per pill.** `pill_element` already turns one `Pill` into a box: label, value and colour. The bar needs one of those for every pill, and the number of pills changes with the order.

**Many at once.** `.child(..)` adds one element. `.children(..)` takes an iterator and adds every element it gives, in order, and hands the box back like any other link in the chain.

**Map, then hand over.** `.iter()` walks a list and `.map(f)` turns each item into something else, as in the Rust track's iterator quests. `.children` takes the iterator itself, so no `collect` is needed. With a list of dividers and a function that draws one, it reads like this:

```rust
.children(self.dividers.iter().map(divider_element))
```

**A function as the map.** `.map` takes any function that fits, not only a closure. `pill_element` takes a `&Pill`, and `.iter()` gives `&Pill`s, so its name goes straight in.

**Names from the kind.** Each pill's box is named after its kind, like `pill-CpuTemp`, by the `debug_selector` line in `pill_element`. That is how the test finds all eight. The next quest explains that line, and you write two like it.

## Your turn

- In `render`, replace the single `.child(..)` with `.children(..)`: every pill in `self.pills`, each through `pill_element`.
- Save. Press Run: the whole default bar, with the hot CPU temperature in red.
