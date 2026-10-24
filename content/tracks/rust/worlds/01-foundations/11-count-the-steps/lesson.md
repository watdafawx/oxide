A theatre has 1 seat in the first row, 2 in the second, 3 in the third, and so on. How many seats do 4 rows have? You could write 1 + 2 + 3 + 4, but the planner has to work for any number of rows. That is a job for a loop.

**Do it once per number.** `for row in 1..=4 { ... }` runs the block once for each number from 1 to 4. Inside the block, `row` is the current number.

**What a range includes.** `1..=4` is 1, 2, 3 and 4, with the end included. `1..4`, without the `=`, stops before 4. That one missing character is the most common mistake with loops.

**A running total.** To add things up, make a total *before* the loop, with `mut` because it changes. Each pass adds to it. After the loop, the total is the answer, on the last line without a semicolon:

```rust
let mut total = 0;
for layer in 1..=3 {
    total += layer * 2;
}
total        // 2 + 4 + 6 = 12
```

**An empty range.** `1..=0` has nothing in it, so the loop never runs and the total stays 0. That is exactly right for zero rows.

**If this is new.** Programming 101 explains counting loops from zero: [Count with for](/quest/basics/04-repetition/02-count-with-for).

## Your turn

- In `seats_in_rows`, loop `row` from 1 through `rows` and add each `row` to the total.
- In `boxes_in_pyramid`, layer 1 has 1 box, layer 2 has 4, layer 3 has 9. Add `layer * layer` for every layer.
- Save. Both objectives turn green.
