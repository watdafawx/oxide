Kinetik's Arrange page lets you drag rows: rows follow the mouse and the others slide out of the way. GPUI handles the dragging; you write what a drop does to the order.

**Drag and drop, wired.** Each row has `.on_drag(DraggedSlot { index, slot }, ..)`, which starts a drag after the mouse moves with the button held and shows a small preview under it. `.on_drop(..)` on every row runs when a `DraggedSlot` is let go over it, with both indices known.

**Move, not swap.** Alt+Up swapped neighbours. A drop moves a row any distance: everything between shifts by one. Take the slot out, then put it in at the target:

```rust
let slot = order.remove(from); order.insert(to, slot);
```

**Guard the edges.** Out-of-range indices or a drop on itself change nothing, so return early. `remove` panics on a bad index; the guard makes that impossible.

**Through the global.** The drop calls `move_slot` inside `Settings::update_global`, so the bar reorders while you watch.

## Your turn

- In `move_slot`, return early for an index out of range or `from == to`.
- `remove` the slot at `from` and `insert` it at `to`.
- Save. Press Run, open Arrange and drag CPU down.
