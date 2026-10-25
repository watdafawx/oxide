The balance screen wants the average weight in a loot table: the sum divided by how many entries there are. So far `sum` learned its type from the function's return type. Here the sum sits in the middle of a calculation, and Rust has nothing to go on.

**Try the obvious thing.** Write `table.iter().map(|d| d.weight).sum() / table.len() as u32` and Rust stops:

```text
error[E0283]: type annotations needed
cannot infer type of the type parameter `S` declared on the method `sum`
```

**Why it asks.** `sum` can build a `u32`, a `u64`, an `f64` and more, and it picks by what you do with the result. Dividing does not settle it. So you name the type on the call itself: `.sum::<u32>()`. That `::<...>` is called the **turbofish**, after its shape `::<>`.

**Fill in only part.** `collect` asks the same question, and often you know the outer type but not the inner one. `collect::<Vec<_>>()` says "a Vec, and you work out what is in it". The `_` means "you fill this in", here from the items in the chain.

```rust
let loot = drops.iter().sum::<u32>() * 2;                      // a u32, then times 2
let doubled = drops.iter().map(|n| n * 2).collect::<Vec<_>>(); // a Vec of whatever map gives
```

**Where else.** You have used the turbofish before without its name: `parse::<u32>()` in the Foundations world asks the same question of `parse`.

## Your turn

- In `average_weight`, write the sum without a turbofish and read the error. Then add `::<u32>`.
- In `full_stacks_worth`, sum as a `u32`, turn it into an `f64` with `as`, and divide by 64.0.
