`count_blocks` tallies the blocks a chunk scan found, gives the right counts, and does three times the work it needs. `src/probe.rs` holds a `Counter`, a map that counts every operation done on it, and the test allows one per block.

**Count the lookups.** For a block seen before, the loop asks `contains` (is it there?), then `get` (what is the number?), then `insert` (store the new number). Each one hashes the name and searches the map for it: three searches for one block.

**One search, a slot.** World 03 counted sales with `entry(key)`: it searches once and hands back the slot for that key, filled or empty. `or_default()` fills an empty slot with the type's default, 0 for a `u32`, and gives you `&mut u32` either way, so `*... += 1` adds to the number in the map.

```rust
let mut drops: HashMap<char, u32> = HashMap::new();
for c in "ddgdg".chars() {          // d = diamond, g = gold
    *drops.entry(c).or_default() += 1;   // one search per drop
}
// d: 3, g: 2
```

**The key must be owned.** An empty slot keeps the key it was given, so `entry` takes the key by value: for `Counter`, a `String`. `block.to_string()` therefore copies every name, even one that is already in the map. That costs one allocation per block; quest 24 shows how to avoid it.

## Your turn

- In `count_blocks`, replace the `if`/`else` with one `entry` call on `counter`, `or_default()`, and `+= 1` on the number.
