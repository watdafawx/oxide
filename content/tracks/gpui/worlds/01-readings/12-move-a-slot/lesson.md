On Kinetik's Arrange page you select a row and press Alt+Up to move it one place left on the bar. In the model that is: swap the slot at `index` with the one just before it.

**Swap in place.** Slices and `Vec`s have `swap(a, b)`: the two entries trade places and nothing else moves. Moving slot 3 up is `order.swap(2, 3)`:

```rust
let mut pair = [StatKind::NetDown, StatKind::NetUp];
pair.swap(0, 1); // now [NetUp, NetDown]
```

**When it cannot move.** Two indexes have no slot before them to swap with. The first slot has nothing before it, and an index past the end points at nothing. Both happen: the first row is selected, or the list just got shorter. Then change nothing and give back `false`, so the page knows to ignore the key.

**Check first.** `swap` panics on an index past the end, and `index - 1` on a `usize` of 0 panics with an overflow, as in the Foundations world. So the check comes before either one, and returns early.

**Then report it.** `true` tells the Arrange page the row moved, so it moves the selection with it and saves the new order.

**Why a slice.** `move_up` takes `&mut [Slot]`, a borrowed list it may change, of any length. It works on the bar's order and, later, on the widget's and the overlay's.

## Your turn

- In `move_up`, first give back `false` for the two indexes that cannot move.
- Otherwise swap the slot with the one before it, and give back `true`.
- Save. Press Run to move the GPU group one step left.
