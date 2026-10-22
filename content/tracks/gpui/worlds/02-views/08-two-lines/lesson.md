On a small taskbar one line of eight stats does not fit. Kinetik's two-line layout stacks them in pairs: CPU usage over CPU temperature, GPU over GPU, and so on, half as wide. You already pair the pills in the model with `two_line_columns`; now draw each pair.

**Columns in a row.** The bar stays a row. In two-line mode its children are columns instead of pills, and each column stacks its pills top to bottom.

**Top to bottom.** `.flex_col()` is `.flex_row()` turned on its side: the children go down, not across. A column is a flex box with `.flex_col()` holding the top pill and, if there is one, the bottom pill.

**Maybe a second pill.** `bottom` is an `Option<Pill>`. An `Option` works as a list of zero or one, so `.children(bottom.map(..))` adds the second pill only when there is one:

```rust
.children(bottom.map(|pill| PillView { pill, show_label }))
```

## Your turn

- In `column`, change the `Column::Pills(top, bottom)` arm to give back a `div().flex().flex_col()` with the top `PillView` as a child and the bottom one through `.children(..)`, then `.into_any_element()`.
- Save. Press Run: a second window shows the two-line bar.
