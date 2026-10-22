Kinetik's widget snaps to screen edges: let go a few pixels from an edge and it lines up exactly. Without snapping, "against the corner" ends up 3 pixels off and looks careless.

**Within a margin.** For each edge, how far is the widget from it? `(window.left - work.left).abs()` is the distance on the left. Within `margin` (12 px), move it flush.

**One side only.** A widget can only touch one side at a time. Check left first; only if it did not snap, check right. The same for top and bottom:

```rust
if (window.left - work.left).abs() <= margin { r.left = work.left; } else if (work.right - window.right).abs() <= margin { r.left = work.right - w; }
```

**Keep the size.** Snapping moves the widget; it never resizes it. Remember the width and height first, and set `right` and `bottom` from the new `left` and `top` at the end.

**The work area.** Snap to the screen minus the taskbar, so snapping to the bottom never tucks the widget under the taskbar.

## Your turn

- In `snap_to_edges`, snap `r.left` to the left or right edge, then `r.top` to the top or bottom edge, each within `margin`.
- Set `r.right` and `r.bottom` from the width and height, and give back `r`.
- Save.
