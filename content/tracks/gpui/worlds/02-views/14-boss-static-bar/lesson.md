Everything the bar is made of exists: the model writes and colours each pill, dividers know their widths, the background is see-through and the text size is clamped. This boss puts it together from scratch: a pill function, the one-line bar, and the two-line bar.

**What is given.** `value_colour`, `pill_background`, `bar_text_size`, `divider_width` and `divider` are written. `StatsBar` holds the items and the settings, and its `render` already builds the outer bar with its height, padding, background and fonts.

**Why a function again.** `PillView` was the right tool for a pill with nothing to remember, and it drew the pill well. The rest of this track keeps the pill as a plain function on purpose: in the Interaction world every pill gets click and hover handlers that change the bar, and a function called from the bar's `render` can take the bar's own `cx: &mut Context<StatsBar>` to build them. A component's `render` only gets `&mut App`.

**One pill.** `pill_element` draws what `PillView` drew, from a `Pill` it is handed by value: a flex row named `pill-<Kind>`, the label box named `label-<Kind>` only when labels are on, then the value box named `value-<Kind>` with its 48-pixel floor and the value's colour.

**One line.** The bar's children are its items in order: `pill_element` for each `BarItem::Pill`, `divider(i, style)` for each divider, with `i` from `.enumerate()` as in Dividers. Both are made `AnyElement` so they fit in one list.

**Two lines.** With `two_lines` on, the children come from `two_line_columns` instead, and `column` draws each: a flex column with one or two pills, or a divider. So `render` ends in an `if` with one `.children(..)` per branch:

```rust
if self.two_lines {
    bar.children(/* one column(..) per two-line column */)
} else {
    bar.children(/* a pill or a divider per item */)
}
```

## Your turn

- Write `pill_element`, then the one-line branch of `render`.
- Write `column`, then the two-line branch of `render`.
- Press Run: the one-line and the two-line bar open in two windows.
