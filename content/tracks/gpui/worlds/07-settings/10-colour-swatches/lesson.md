In Kinetik's icon picker you can give any stat its own colour from 15 swatches. Here the swatches sit under the Arrange list and colour the selected row's stat.

**Colours as numbers.** `SWATCHES` holds the 15 colours as `0xRRGGBB` numbers. `Settings::colours` maps a stat to its number; the bar draws a stat's value in its colour, unless it is a warning, which stays red.

**Change the global.** `Settings::update_global(cx, |s, _| ..)` lends `&mut Settings` to the closure and, after it, tells every watcher, so the bar repaints. Setting the gap that way would be:

```rust
Settings::update_global(cx, |s, _| { s.gap = 12.0; });
```

**Insert a colour.** For a swatch, the change is one `insert` into `s.colours`: the swatch's `kind` as the key, its `colour` as the value. An older colour for that stat is replaced.

**No `this` needed.** The listener changes the global, not the window's fields, so it ignores `this` with `_`: `|_, _, _, cx|`. It takes `kind` and `colour` with `move`.

**Your colour, or none.** A stat without a colour of its own uses the bar's text colour, which follows light and dark mode. Dividers have no colour, so the Arrange page shows swatches only when a stat row is selected.

## Your turn

- In `swatch`, add `.on_click(cx.listener(move |_, _, _, cx| { .. }))`.
- Inside, insert `colour` for `kind` with `Settings::update_global`, then notify.
- Save. Press Run, open Arrange and colour CPU.
