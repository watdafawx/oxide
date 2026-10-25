`longest` finds the longest name in a list. It clones every name that takes the lead, and on a long list that is many copies of text that is thrown away a moment later. `src/probe.rs` holds `Name`, whose `clone` is counted, and the test allows one.

**A reference is free.** A `&Name` is an address, a few bytes; copying it or pointing it at another name copies no text and allocates nothing. A `clone()` of a `Name` copies its whole `String` onto the heap. While you are only comparing, a reference is all you need.

**Clone what you return.** The search looks at every name but keeps just one. So `best` can be a reference that moves from leader to leader, and the single clone happens at the end, for the value that leaves the function as an owned `Name`.

```rust
let words = vec![String::from("apple"), String::from("banana"), String::from("fig")];
let mut shortest: &String = &words[0];   // a reference: copying it is free
for w in &words {
    if w.len() < shortest.len() {
        shortest = w;                     // point at the new leader, no clone
    }
}
let owned: String = shortest.clone();     // exactly one copy, at the end
```

**Why return an owned Name at all.** The caller may keep the result after the list is gone, so the function hands back its own copy. Returning `Option<&Name>` would cost nothing, but would tie the answer to the list's lifetime (world 02).

## Your turn

- In `longest`, keep `best` as a `&Name` while searching.
- Clone it once, in the value you return.
