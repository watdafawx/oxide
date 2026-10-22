Kinetik's widget has four styles: Bars, Graphs, Bars and graphs, or Minimal. Each answers two yes-or-no questions for every stat: draw its usage bar? draw its graph?

**Two answers at once.** `parts(style)` gives a pair `(bars, graphs)`. The widget's `render` unpacks it with `let (show_bars, show_graphs) = parts(..)` and uses each half in a `.when(..)`.

**One arm per style.** Four styles, four arms, each a pair of `bool`s:

```rust
WidgetStyle::Minimal => (false, false),
```

**From Settings.** The style lives in `Settings::widget`, so changing it on the Widget page redraws the widget at once, through the same `observe_global` as the bar.

**Values always.** Every style keeps the label and the number. Minimal is exactly that: a compact list, nothing drawn under it.

## Your turn

- In `parts`, `match` on `style` and give the pair for each of the four.
- Save.
