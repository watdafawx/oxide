**Why now:** you can count allocations and operations. Now fix four spots in the chunk indexer, each with a different habit. The tests do not time anything: they read a meter, so a pass means the pattern is right on every machine.

```rust
// 1. One lookup, not three: entry finds or creates the slot, then you bump it
*tally.entry(tag).or_default() += 1;          // instead of contains_key + get + insert
// (the test's Counter wants an owned key: tally.entry(tag.to_string()))

// 2. Compare by reference, clone once at the end
let mut best: &String = &words[0];            // a reference: free to copy
for w in &words { if w.len() > best.len() { best = w; } }
let owned = best.clone();                     // the only clone

// 3. Reserve once
let mut out: Vec<u64> = Vec::with_capacity(n);

// 4. Iterators allocate nothing: no collect, no to_string
let long = text.split_whitespace().filter(|w| w.len() >= 5).count();
```

- `entry` hashes the key once and gives you a handle to the slot.
- References are free; `clone` is a copy of the data. Clone what you **return**, not what you inspect.
- `with_capacity` turns "grow, copy, grow, copy" into one allocation.
- Iterator adaptors work on borrowed data and build nothing. `collect` is where allocation happens.
- `src/probe.rs` holds the meters. Nothing to edit there.

## Your turn

In `src/lib.rs`: `count_blocks`, `longest_name`, `chunk_areas`, `count_long_names`. Each currently works but breaks its budget.
