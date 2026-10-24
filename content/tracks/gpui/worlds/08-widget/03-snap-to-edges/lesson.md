Kinetik's widget snaps to the screen's edges: let go a few pixels from an edge and it lines up exactly. Without snapping, "in the corner" ends up 3 pixels off and looks careless. `snap_to_edges` gets the widget's rectangle where you let go and gives back where it should end up.

**The work area.** Snap to the screen minus the taskbar, `work`, not to the whole screen. Then snapping to the bottom never tucks the widget under the taskbar.

**Within a margin.** For each edge, how far is the widget from it? On the left that is `window.left - work.left`. `.abs()` drops the sign, so 10 and -10 both count as 10 pixels away. Within `margin` (12 px), move the widget flush against that edge.

**One side per direction.** A 300 px widget cannot touch both the left and the right edge of a 2560 px screen. Check left first, and check right only if left did not snap: `if .. { } else if .. { }`. Then the same for top and bottom.

**Work on a copy.** The function gets `window` and must not lose the old numbers while it changes things. So start with `let mut r = window;`, a copy you may change (`Rect` is `Copy`), and keep reading the old edges from `window`.

**Keep the size.** Snapping moves the widget; it never resizes it. Note the width and height first, move `left` and `top`, and set `right` and `bottom` from them at the end. Here is the top edge alone; your version also does the bottom, then left and right:

```rust
let mut r = window;
let h = window.height();
if (window.top - work.top).abs() <= margin {
    r.top = work.top;
}
r.bottom = r.top + h;
```

## Your turn

- In `snap_to_edges`, make a copy `r`, and note the widget's width and height.
- Snap `r.left` to the left edge, or else to the right edge, within `margin`. Do the same for `r.top` with top and bottom.
- Set `r.right` and `r.bottom` from the width and height, and give back `r`.
