The "sort chest" button puts the biggest stacks first, so the useful stuff is on top. `sort()` worked on plain numbers and names, but an `ItemStack` has two fields, and Rust has no idea which one you mean.

**Try the obvious thing.** `stacks.sort()` on a `Vec<ItemStack>` does not build:

```text
error[E0277]: the trait bound `ItemStack: Ord` is not satisfied
```

**What the red means.** `Ord` is the trait "has one natural order". Numbers and text have it; your struct does not, because there are several sensible orders. So you give the rule yourself with `.sort_by(closure)`. The closure gets two items, `a` and `b`, and says which comes first.

**Say which comes first.** The answer is an **`Ordering`**, an enum with three variants: `Less` (a goes first), `Equal` and `Greater` (b goes first). You rarely write them yourself: `a.count.cmp(&b.count)` compares two numbers and gives the right one. `cmp` takes the other side by reference, hence the `&`.

```rust
players.sort_by(|a, b| a.level.cmp(&b.level));    // lowest level first
players.sort_by(|a, b| b.level.cmp(&a.level));    // highest first: a and b swapped
```

**Flip it.** Swapping `a` and `b` in the comparison reverses the order. That is the whole trick for "biggest first".

**For the curious.** `sort_by_key(|s| s.count)` is a shortcut for the smallest-first case: give it the key, as with `max_by_key`.

## Your turn

- In `biggest_first`, try `stacks.sort()` and read the error. Then sort by `count` with `a` and `b` swapped.
- In `by_name`, sort by `name`, a to z.
