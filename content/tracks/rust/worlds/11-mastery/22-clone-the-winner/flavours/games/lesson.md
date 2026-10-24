`longest_name` picks the longest wood type for the indexer's legend column width. It clones every name that takes the lead, and on a modpack's hundreds of wood types that is many copies of text thrown away a moment later. `src/probe.rs` holds `Name`, whose `clone` is counted, and the test allows one.

**A reference is free.** A `&Name` is an address, a few bytes; copying it or pointing it at another name copies no text and allocates nothing. A `clone()` of a `Name` copies its whole `String` onto the heap. While you are only comparing, a reference is all you need.

**Clone what you return.** The search looks at every name but keeps just one. So `best` can be a reference that moves from leader to leader, and the single clone happens at the end, for the value that leaves the function as an owned `Name`.

```rust
let ores = vec![String::from("diamond"), String::from("coal"), String::from("redstone")];
let mut shortest: &String = &ores[0];   // a reference: copying it is free
for ore in &ores {
    if ore.len() < shortest.len() {
        shortest = ore;                  // point at the new leader, no clone
    }
}
let owned: String = shortest.clone();    // exactly one copy, at the end
```

**Why return an owned Name at all.** The caller may keep the result after the list is gone, so the function hands back its own copy. Returning `Option<&Name>` would cost nothing, but would tie the answer to the list's lifetime (world 02).

## Your turn

- In `longest_name`, keep `best` as a `&Name` while searching.
- Clone it once, in the value you return.
