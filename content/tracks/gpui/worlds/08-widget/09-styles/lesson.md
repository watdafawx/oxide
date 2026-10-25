Kinetik's widget has four styles: Bars, Graphs, Bars and graphs, or Minimal. Each one answers two yes-or-no questions for every stat: draw its usage bar? Draw its graph? `parts` gives both answers at once.

**Two answers in one value.** A **tuple** holds a few values side by side, written in parentheses: `(true, false)`. `parts(style)` gives the pair `(bars, graphs)`, and the widget's `render` unpacks it into two names in one line: `let (show_bars, show_graphs) = parts(style);`.

**One arm per style.** A `match` arm can give back a tuple like any other value. Four styles, four arms, each a pair of `bool`s. Here the same idea answers two questions about where the widget sits:

```rust
let (on_top, behind_windows) = match placement {
    Placement::Desktop => (false, true),
    Placement::Normal => (false, false),
    Placement::OnTop => (true, false),
};
```

**Values always.** Every style keeps the label and the number. Minimal is exactly that: a compact list with nothing drawn under it, so it draws neither bars nor graphs.

**From Settings.** The style lives in `Settings::widget`, so changing it on the Widget page redraws the widget at once, through the same `observe_global` as the bar.

## Your turn

- In `parts`, `match` on `style` and give back `(bars, graphs)` for each of the four styles.
- Save.
