The recipe book now answers `Some(4)` or `None`. Your calculator wants plain numbers: how many planks from 3 crafts. To do maths, the number has to come out of its wrapper, and you have to decide what a missing answer means.

**Try the obvious thing.** Add 1 to `recipe_output("planks")` and Rust refuses, because `Some(4)` is not a number:

```text
error[E0369]: cannot add `{integer}` to `Option<u32>`
```

**Give a fallback.** `.unwrap_or(0)` opens the wrapper. If there is a value inside, you get it. If it is `None`, you get the fallback you wrote in the brackets instead.

**Choose it on purpose.** The fallback is a decision. For counts, `0` usually means "none of them", which is fine. For other jobs a different number, or a different approach, is better. Rust makes you choose rather than guess.

```rust
let burn = fuel_burn("coal").unwrap_or(0);    // 80
let dirt = fuel_burn("dirt").unwrap_or(0);    // 0
```

**One to leave alone.** There is also `.unwrap()`, which has no fallback and crashes the program on `None`. Leave it alone while you are learning; `unwrap_or` is the safe way.

## Your turn

- In `output_or_zero`, ask `recipe_output` and fall back to 0.
- In `total_output`, do the same and multiply by `crafts`.
- Save. Both objectives turn green.
