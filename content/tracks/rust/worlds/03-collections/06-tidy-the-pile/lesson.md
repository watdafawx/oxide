Today's orders came in as a messy list: `mug, pen, mug, lamp, pen`. The supplier wants each product once, in alphabetical order. Two `Vec` methods do the tidying, and the order you call them in matters.

**Sort it.** `orders.sort()` puts the list in order, smallest first: numbers by size, text a to z. It changes the list in place, so the list must be `mut`. A function that owns its list can write `let mut orders = orders;`, the same trick as `while` used in the Foundations world.

**Drop the repeats.** `orders.dedup()` removes an item when it equals the one right before it. That is all it checks, so `[mug, pen, mug]` stays as it is: the two mugs are not neighbours.

```rust
let mut sizes = vec![3, 1, 3, 2, 1];
sizes.sort();      // [1, 1, 2, 3, 3]: repeats are now neighbours
sizes.dedup();     // [1, 2, 3]
```

**Sort first.** After a sort, equal items sit next to each other, so `dedup` catches all of them. Sort, then dedup: that pair is the usual way to say "each one once, in order".

**Capitals first.** Text sorts by character code, and every capital comes before every small letter, so `"Zebra pen"` sorts before `"apple"`. Keep product names in one case if the order should look alphabetical to a person.

## Your turn

- In `sorted_orders`, make the list `mut` and sort it.
- In `unique_orders`, sort it and then dedup it.
