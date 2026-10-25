After a night of fighting, the loot pile reads `bone, arrow, bone, string, arrow`. The loot screen should list each item once, in alphabetical order. Two `Vec` methods do the tidying, and the order you call them in matters.

**Sort it.** `drops.sort()` puts the list in order, smallest first: numbers by size, text a to z. It changes the list in place, so the list must be `mut`. A function that owns its list can write `let mut drops = drops;`, the same trick as `while` used in the Foundations world.

**Drop the repeats.** `drops.dedup()` removes an item when it equals the one right before it. That is all it checks, so `[bone, arrow, bone]` stays as it is: the two bones are not neighbours.

```rust
let mut tiers = vec![3, 1, 3, 2, 1];
tiers.sort();      // [1, 1, 2, 3, 3]: repeats are now neighbours
tiers.dedup();     // [1, 2, 3]
```

**Sort first.** After a sort, equal items sit next to each other, so `dedup` catches all of them. Sort, then dedup: that pair is the usual way to say "each one once, in order".

**Capitals first.** Text sorts by character code, and every capital comes before every small letter, so `"Zombie_head"` sorts before `"arrow"`. Item ids are all small letters, which is one reason the game keeps them that way.

## Your turn

- In `sorted_loot`, make the list `mut` and sort it.
- In `unique_loot`, sort it and then dedup it.
