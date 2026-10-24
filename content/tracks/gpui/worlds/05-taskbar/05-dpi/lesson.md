At 150 % display scaling, Windows reports the taskbar as 72 pixels tall: real screen pixels, called physical. GPUI places windows in logical pixels, which scaling multiplies back up. Hand GPUI the physical numbers and the bar comes out half again too big and in the wrong place.

**Why screens scale.** A 4K laptop screen packs its pixels so tightly that text drawn 12 pixels tall is too small to read. So Windows has a scale setting, 100 %, 125 %, 150 % and up, and apps draw everything that much bigger. The number behind it is **DPI**, dots per inch: 96 DPI is 100 %, 144 DPI is 150 %.

**Two kinds of pixel.** Physical pixels are the screen's own dots. Logical pixels are what apps lay out in; at 150 % one logical pixel covers 1.5 physical ones. `Taskbar::scale` holds that factor, from `GetDpiForWindow` divided by 96. Win32 calls like `GetWindowRect` speak physical; GPUI's `px(..)` speaks logical.

**Divide everything.** Position and size both scale, so divide each of the four numbers by `scale`. The numbers are `i32`, so turn each into an `f32` first with `as f32`, or the division drops the fraction. The other way round, a logical width back to physical:

```rust
let physical = (px_wide * scale).round() as i32;
```

**Then open there.** `main.rs` now docks for real: `find_taskbar`, `dock_bar`, `to_logical`, and `bar_window_options` with those bounds. On an auto-hidden taskbar it opens just above where the taskbar would be, so you can see it.

**The 8 pixels.** When this track was tested, GPUI added Windows' invisible 8-pixel frame outside the bounds you ask for. The bounds are the drawn area, which is what you want; never compare them with `GetWindowRect`.

## Your turn

- In `to_logical`, divide the left, top, width and height by `scale` before each `px(..)`.
- Save. Press Run: the bar opens left of your tray, at the taskbar's height.
