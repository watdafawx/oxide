On Kinetik's Arrange page you select a row and press Alt+Up to move it one place left on the bar. In the model that is: swap the slot at `index` with the one just before it.

**Swap in place.** Slices and `Vec`s have `swap(a, b)`: the two entries trade places and nothing else moves. Moving slot 3 up is `order.swap(2, 3)`.

**When it cannot move.** The first slot has nothing before it. An index past the end points at nothing. Both happen: the first row is selected, or the list just got shorter. In those cases change nothing and give back `false`, so the page knows to ignore the key:

```rust
if index == 0 || index >= order.len() { return false; }
```

**Then report it.** `true` tells the Arrange page the row moved, so it moves the selection with it and saves the new order.

**Why a slice.** `move_up` takes `&mut [Slot]`, a borrowed list it may change, of any length. It works on the bar's order and, later, on the widget's and the overlay's.

## Your turn

- In `move_up`, replace the TODO with the `if` that returns `false`.
- Under it, swap `index - 1` and `index`, then give back `true`.
- Save. Press Run to move the GPU group one step left.
