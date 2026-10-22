In Kinetik's icon picker you can give any stat its own colour from 15 swatches. Here the swatches sit under the Arrange list and colour the selected row's stat.

**Colours as numbers.** `SWATCHES` holds the 15 colours as `0xRRGGBB` numbers. `Settings::colours` maps a stat to its number; the bar draws a stat's value in its colour, unless it is a warning, which stays red.

**Change the global.** A click inserts the colour for the stat. `Settings::update_global(cx, |s, _| ..)` lends `&mut Settings` and, after the closure, tells every watcher, so the bar repaints:

```rust
Settings::update_global(cx, |s, _| { s.colours.insert(kind, colour); });
```

**Which stat.** The Arrange page draws swatches only when the selected row is a stat, and hands that stat to each swatch. Dividers have no colour here.

**Your colour, or none.** A stat without a colour of its own uses the bar's text colour, which follows light and dark mode.

## Your turn

- In `swatch`, add `.on_click(cx.listener(move |_, _, _, cx| { .. }))`.
- Inside, insert `colour` for `kind` with `Settings::update_global`, then notify.
- Save. Press Run, open Arrange and colour CPU.
