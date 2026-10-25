Kinetik's Arrange page lets you drag rows: rows follow the mouse and the others slide out of the way. GPUI handles the dragging; you write what a drop does to the order.

**Drag and drop, wired.** Each row has `.on_drag(DraggedSlot { index, slot }, ..)`, which starts a drag after the mouse moves with the button held and shows a small preview under it. `.on_drop(..)` on every row runs when a `DraggedSlot` is let go over it, with both indices known.

**Move, not swap.** `move_up` from Readings swapped neighbours. A drop moves a row any distance: everything between shifts by one. `Vec::remove(i)` takes the item out and hands it back, and `Vec::insert(i, item)` puts it in at `i`, as in [Rust world 03](/quest/rust/03-collections/05-move-an-item). Moving the first item to the end, for example:

```rust
let first = list.remove(0);
list.push(first);
```

**Guard the edges.** Out-of-range indices or a drop on itself change nothing, so return early. `remove` panics on a bad index; the guard makes that impossible.

**Through the global.** The drop calls `move_slot` inside `Settings::update_global`, so the bar reorders while you watch.

## Your turn

- In `move_slot`, return early for an index out of range or `from == to`.
- `remove` the slot at `from` and `insert` it at `to`.
- Save. Press Run, open Arrange and drag CPU down.
