Everything the bar is made of exists: the model writes and colours each pill, dividers know their widths, the background is see-through and the text size is clamped. This boss puts it together from scratch: a pill function, the one-line bar, and the two-line bar.

**What is given.** `value_colour`, `pill_background`, `bar_text_size`, `divider_width` and `divider` are written. `StatsBar` holds the items and the settings, and its `render` already builds the outer bar with its height, padding, background and fonts.

**One pill.** `pill_element` draws one pill as a flex row named `pill-<Kind>`. Inside: the label box named `label-<Kind>`, only when labels are on, then the value box named `value-<Kind>` with a 48-pixel minimum width and the value's colour.

**One line.** The bar's children are its items in order: a pill for each `BarItem::Pill`, a `divider(i, style)` for each divider, both made `AnyElement` so they fit in one list.

**Two lines.** With `two_lines` on, the children come from `two_line_columns` instead, and `column` draws each: a `flex_col` box with one or two pills, or a divider:

```rust
if self.two_lines {
```

## Your turn

- Write `pill_element`, then the one-line branch of `render`.
- Write `column`, then the two-line branch of `render`.
- Press Run: the one-line and the two-line bar open in two windows.
