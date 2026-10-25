A staircase has 1 block in the first step, 2 in the second, 3 in the third, and so on. How many blocks do 4 steps need? You could write 1 + 2 + 3 + 4, but the calculator has to work for any number of steps. That is a job for a loop.

**Do it once per number.** `for step in 1..=4 { ... }` runs the block once for each number from 1 to 4. Inside the block, `step` is the current number.

**What a range includes.** `1..=4` is 1, 2, 3 and 4, with the end included. `1..4`, without the `=`, stops before 4. That one missing character is the most common mistake with loops.

**A running total.** To add things up, make a total *before* the loop, with `mut` because it changes. Each pass adds to it. After the loop, the total is the answer, on the last line without a semicolon:

```rust
let mut total = 0;
for layer in 1..=3 {
    total += layer * 2;
}
total        // 2 + 4 + 6 = 12
```

**An empty range.** `1..=0` has nothing in it, so the loop never runs and the total stays 0. That is exactly right for zero steps.

**If this is new.** Programming 101 explains counting loops from zero: [Count with for](/quest/basics/04-repetition/02-count-with-for).

## Your turn

- In `staircase_blocks`, loop `step` from 1 through `steps` and add each `step` to the total.
- In `pyramid_blocks`, layer 1 has 1 block, layer 2 has 4, layer 3 has 9. Add `layer * layer` for every layer.
- Save. Both objectives turn green.
