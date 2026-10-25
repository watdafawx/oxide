The weekly report lists stock lines with the most units first, so the big numbers are on top. `sort()` worked on plain numbers and names, but a `StockLine` has two fields, and Rust has no idea which one you mean.

**Try the obvious thing.** `lines.sort()` on a `Vec<StockLine>` does not build:

```text
error[E0277]: the trait bound `StockLine: Ord` is not satisfied
```

**What the red means.** `Ord` is the trait "has one natural order". Numbers and text have it; your struct does not, because there are several sensible orders. So you give the rule yourself with `.sort_by(closure)`. The closure gets two items, `a` and `b`, and says which comes first.

**Say which comes first.** The answer is an **`Ordering`**, an enum with three variants: `Less` (a goes first), `Equal` and `Greater` (b goes first). You rarely write them yourself: `a.qty.cmp(&b.qty)` compares two numbers and gives the right one. `cmp` takes the other side by reference, hence the `&`.

```rust
staff.sort_by(|a, b| a.age.cmp(&b.age));    // youngest first
staff.sort_by(|a, b| b.age.cmp(&a.age));    // oldest first: a and b swapped
```

**Flip it.** Swapping `a` and `b` in the comparison reverses the order. That is the whole trick for "biggest first".

**For the curious.** `sort_by_key(|l| l.qty)` is a shortcut for the smallest-first case: give it the key, as with `max_by_key`.

## Your turn

- In `most_first`, try `lines.sort()` and read the error. Then sort by `qty` with `a` and `b` swapped.
- In `by_name`, sort by `name`, a to z.
