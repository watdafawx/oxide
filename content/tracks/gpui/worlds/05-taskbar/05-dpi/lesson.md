At 150 % display scaling, Windows reports the taskbar as 72 pixels tall: real screen pixels, called physical. GPUI places windows in logical pixels, which scaling multiplies back up. Hand GPUI the physical numbers and the bar comes out half again too big and in the wrong place.

**Two kinds of pixel.** Physical pixels are the screen's own dots. Logical pixels are what apps lay out in; at 150 % one logical pixel covers 1.5 physical ones. `Taskbar::scale` holds that factor, from `GetDpiForWindow / 96`.

**Divide everything.** Position and size both scale. Divide each of the four numbers by `scale`:

```rust
point(px(rect.left as f32 / scale), px(rect.top as f32 / scale))
```

**Then open there.** `main.rs` now docks for real: `find_taskbar`, `dock_bar`, `to_logical`, and `bar_window_options` with those bounds. On an auto-hidden taskbar it opens just above where the taskbar would be, so you can see it.

**The 8 pixels.** Testing showed GPUI adds Windows' invisible 8-pixel frame outside the bounds you ask for. The bounds are the drawn area, which is what you want; never compare them with `GetWindowRect`.

## Your turn

- In `to_logical`, divide the left, top, width and height by `scale` before `px(..)`.
- Save. Press Run: the bar opens left of your tray, at the taskbar's height.
