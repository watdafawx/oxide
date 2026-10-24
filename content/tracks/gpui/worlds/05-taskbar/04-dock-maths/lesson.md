`find_taskbar` gives three rectangles: the taskbar, its tray, its monitor. The bar's own rectangle follows from those: right edge just left of the tray, top and bottom the taskbar's, width the bar's own. Kinetik's General page adds a "Shift left" setting for PCs where pinned icons would sit under the bar.

**Screen coordinates.** Win32 measures from the top-left corner of the primary monitor: x grows to the right, y grows downwards. A `Rect` holds `left`, `top`, `right` and `bottom` in those numbers, and `width()` and `height()` subtract them.

**The right edge.** The bar ends `TRAY_GAP` (8 px) left of the tray's left edge, moved further by `shift_left`. Its left edge is that minus the bar's width; top and bottom come from the visible taskbar.

**Only the visible part.** An auto-hidden taskbar is still there, slid almost off the screen. On the PC this track was written on, it sits at y 1438 to 1486 on a 1440-high monitor: 2 pixels peek out. `a.intersect(&b)` gives the part two rectangles share, or `None` when they do not touch. Here it finds how much of a window is on its monitor:

```rust
let on_screen = window.intersect(&monitor)?;
```

**Hidden means no bar.** Under 4 visible pixels, give back `None`: the bar hides with the taskbar, as Kinetik does. When you point at the screen edge and the taskbar slides up, the next check docks the bar again.

**Pure maths.** No Windows calls here, so the tests use a made-up 2560 by 1440 monitor and check exact numbers.

## Your turn

- In `dock_bar`, take the part of the taskbar that is on its monitor, and return `None` when it is under 4 px high.
- Work out `right` from the tray, the gap and `shift_left`, and give back the rectangle.
- Save.
