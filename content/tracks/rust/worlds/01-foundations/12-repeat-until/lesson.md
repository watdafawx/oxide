How many scoops does it take to empty a pile of 10 sheets if one scoop takes 3? You do not know in advance, so a counted `for` is the wrong tool. You want to keep going until the pile is gone.

**Repeat while it is true.** `while pile > 0 { ... }` checks the condition, runs the block, checks again, and stops the first time the condition is false.

**Change something every turn.** Something inside the block has to move the condition toward false. If the pile never shrinks, the loop never ends and your program hangs. Make that the first thing you check.

**Count the turns.** Keep a counter next to the loop, with `mut`, and add 1 each turn. When the loop ends, the counter is how many turns it took:

```rust
let mut left = 10;
let mut turns = 0;
while left > 0 {
    left -= 3;
    turns += 1;
}
turns        // 10 -> 7 -> 4 -> 1 -> -2: four turns
```

**Signed on purpose.** The last scoop takes more than is left, so `left` ends below zero: -2. A `u32` cannot do that. That is why a pile that can overshoot is an `i32`. (Quest 7's `saturating_sub` would also work: it stops at 0 instead. Here you want to see the -2, so the pile stays signed.)

**Parameters stay put.** A **parameter** is one of the inputs named in a function's first line, inside the `( )`. It cannot change, just like a plain `let`. To change one, make a changeable copy with the same name: `let mut sheets = sheets;`. That is shadowing, and it needs no new rule.

## Your turn

- In `scoops_to_empty`, make a counter and a pile you can change, and loop while the pile is above 0.
- In `boxes_filled`, keep filling a box while there are at least `per_box` sheets left.
- Save. Both objectives turn green.
