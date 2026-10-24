`word_count` gives the right counts and does three times the work it needs. `src/probe.rs` holds a `Counter`, a map that counts every operation done on it, and the test allows one per word.

**Count the lookups.** For a word seen before, the loop asks `contains` (is it there?), then `get` (what is the number?), then `insert` (store the new number). Each one hashes the word and searches the map for it: three searches for one word.

**One search, a slot.** World 03 counted sales with `entry(key)`: it searches once and hands back the slot for that key, filled or empty. `or_default()` fills an empty slot with the type's default, 0 for a `u32`, and gives you `&mut u32` either way, so `*... += 1` adds to the number in the map.

```rust
let mut letters: HashMap<char, u32> = HashMap::new();
for c in "banana".chars() {
    *letters.entry(c).or_default() += 1;   // one search per letter
}
// b: 1, a: 3, n: 2
```

**The key must be owned.** An empty slot keeps the key it was given, so `entry` takes the key by value: for `Counter`, a `String`. `word.to_string()` therefore copies every word, even one that is already in the map. That costs one allocation per word; quest 24 shows how to avoid it.

## Your turn

- In `word_count`, replace the `if`/`else` with one `entry` call on `counter`, `or_default()`, and `+= 1` on the number.
