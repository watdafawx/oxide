A `&mut u32` is an arrow pointing at a number somewhere else. Write `score += 5` and you are asking Rust to add 5 to the arrow, not to the number it points at. Rust says no.

**Try the obvious thing.** With `score: &mut u32`, the line `score += n;` stops the build. The help line underneath already names the fix:

```text
error[E0368]: binary assignment operation `+=` cannot be applied to type `&mut u32`
```

**Follow the arrow.** `*score` means "the value `score` points at". `*score += 5` adds to the number itself, in the caller's variable. Following a reference like this is called **dereferencing**, and `*` is the **dereference** operator.

**Same when reading.** `for s in &board.scores` lends each score as a `&u32`. To compare it with a plain number, follow it first: `*s > limit`.

```rust
fn level_up(level: &mut u32) {
    *level += 1;
}

fn count_maxed(team: &Team) -> usize {
    let mut n = 0;
    for l in &team.levels {
        if *l == 10 {
            n += 1;
        }
    }
    n
}
```

**Dots follow for you.** Methods and fields follow the arrow on their own: `bag.items.len()` works on a `&Backpack`, and `s.push_str` on a `&mut String`. That is why you needed no `*` until now.

**An arrow to an arrow.** A reference can point at another reference, written `&&u32`, and `**r` follows both. You will meet it when looping over borrowed lists in the next world; the fix is always one more `*`.

## Your turn

- In `add_points`, add `n` to the number `score` points at.
- In `count_over`, count the scores bigger than `limit`. Follow each `s` with `*` to compare it.
