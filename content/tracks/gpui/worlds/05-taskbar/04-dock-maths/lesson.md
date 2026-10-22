`find_taskbar` gives three rectangles: the taskbar, its tray, its monitor. The bar's own rectangle follows from those: right edge just left of the tray, top and bottom the taskbar's, width the bar's own. Kinetik's General page adds a "Shift left" setting for PCs where pinned icons would sit under the bar.

**The right edge.** The bar ends `TRAY_GAP` (8 px) left of the tray, moved further by `shift_left`. Its left edge is that minus the bar's width.

**Only the visible part.** An auto-hidden taskbar is still there, slid almost off the screen. On the PC this track was written on, it sits at y 1438 to 1486 on a 1440-high monitor: 2 pixels peek out. `intersect` gives the part on the monitor:

```rust
let visible = taskbar.rect.intersect(&taskbar.monitor)?;
```

**Hidden means no bar.** Under 4 visible pixels, give back `None`: the bar hides with the taskbar, as Kinetik does. When you point at the screen edge and the taskbar slides up, the next check docks the bar again.

**Pure maths.** No Windows calls here, so the tests use a made-up 2560 by 1440 monitor and check exact numbers.

## Your turn

- In `dock_bar`, take the visible part with `intersect(..)?` and return `None` under 4 px high.
- Work out `right` from the tray, the gap and `shift_left`, and give back the rectangle.
- Save.
