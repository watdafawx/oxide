How many swings does it take to break a block with 10 hardness if your pickaxe does 3 damage per swing? You do not know in advance, so a counted `for` is the wrong tool. You want to keep swinging until the block is gone.

**Repeat while it is true.** `while hp > 0 { ... }` checks the condition, runs the block, checks again, and stops the first time the condition is false.

**Change something every turn.** Something inside the block has to move the condition toward false. If the block never loses hardness, the loop never ends and your program hangs. Make that the first thing you check.

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

**Signed on purpose.** The last swing does more damage than is left, so `left` ends below zero: -2. A `u32` cannot do that. That is why a number that can overshoot is an `i32`.

**Parameters stay put.** A parameter cannot change, just like a plain `let`. To change one, make a changeable copy with the same name: `let mut fuel = fuel;`. That is shadowing, and it needs no new rule.

## Your turn

- In `swings_to_break`, make a counter and a hardness you can change, and loop while it is above 0.
- In `smelts_before_empty`, keep smelting while there is at least `per_smelt` fuel left.
- Save. Both objectives turn green.
