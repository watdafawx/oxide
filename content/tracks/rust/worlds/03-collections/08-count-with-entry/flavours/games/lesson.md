After a mob farm runs all night, the kill log is a list of drops: `bone, flesh, bone, bone`. The loot screen wants a count per item. For each drop you add one to that item's count, and the first time an item shows up it has no count yet.

**One call for both cases.** `counts.entry(key)` is the slot in the map for that key, whether it is filled or not. `.or_insert(0)` fills an empty slot with 0. Either way you end up holding the count for that key, ready to change.

**Change it where it lives.** `or_insert` hands back a `&mut u32`: a mutable reference to the number inside the map, not a copy. Put `*` in front to follow it, the same `*` as before, and `+= 1` changes the number in the map itself.

```rust
let mut kills = HashMap::new();
for mob in ["zombie", "creeper", "zombie"] {
    *kills.entry(mob.to_string()).or_insert(0) += 1;
}
// zombie: 2, creeper: 1
```

**Why not get, then insert?** You could `get` the old count, add one and `insert` it back. That is two lookups and a `match` for the missing case. `entry` finds the slot once and handles both cases in one line.

**Why to_string.** The map owns its keys, so `entry` wants an owned `String`, not a borrowed `&str`. `item.to_string()` makes that copy; `get` can still look up with a plain `&str`.

## Your turn

- In `count_drops`, add one to each item's count inside the loop.
- In `add_to_chest`, add `amount` to the item's count in `chest`. A new item starts at 0.
