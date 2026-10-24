On a small taskbar one line of eight stats does not fit. Kinetik's two-line layout stacks them in pairs: CPU usage over CPU temperature, GPU over GPU, and so on, half as wide. You already pair the pills in the model with `two_line_columns`; now draw each pair.

**Columns in a row.** The bar stays a row. In two-line mode its children are columns instead of pills, and each column stacks its pills top to bottom.

**Top to bottom.** `.flex_col()` is `.flex_row()` turned on its side: the children go down, not across. A column is a flex box with `.flex_col()` holding the top pill and, if there is one, the bottom pill.

**Maybe a second one.** `bottom` is an `Option<Pill>`. An `Option` works as a list of zero or one, so `.children(..)` takes it directly once `.map(..)` has turned the pill inside into an element. An optional icon in front of a label would look like this:

```rust
.children(icon.map(|name| div().child(name)))
```

**One type again.** `column` returns `AnyElement`, because its other arm gives back a divider, a different type. So the column box ends with `.into_any_element()`, as each arm in the bar's `render` did in Dividers.

**Smaller text.** Two lines of 13-pixel text do not fit in 32 pixels. In two-line mode the starter's `render` drops to extra-small text with a 15-pixel line height.

## Your turn

- In `column`, change the `Column::Pills(top, bottom)` arm to give back a flex column: the top `PillView`, then the bottom one if there is one.
- End it with `.into_any_element()`, like the divider arm.
- Save. Press Run: a second window shows the two-line bar.
