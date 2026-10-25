`tally_blocks` counts every block name in a scan log, ignoring case: `Stone`, `stone,` and `STONE` are one block. It allocates a new lowercase `String` for every name it reads, 300 allocations for 300 names, though only 5 of them are different. This is the last tool the boss needs.

**to_lowercase makes a new String.** Every call asks the heap for a fresh block, fills it, and the block is thrown away a moment later when the name turns out to be known. What you need is somewhere to put the lowercase letters that you can use again.

**One buffer, cleared.** Make one `String` before the loop, `buf`. For each name, `buf.clear()` empties it but keeps the room it has, so the next name usually fits without a new allocation. `buf.extend(...)` then appends characters to it.

**Why flat_map.** `char::to_lowercase` gives back not one `char` but a few: a handful of letters lowercase into two characters, such as `İ`, a capital I with a dot. So each character becomes a short run of characters, and `flat_map` joins the runs into one stream for `extend`: `name.chars().flat_map(char::to_lowercase)`.

**Look up by &str, copy only when new.** `entry` needs an owned `String` key every time (quest 21). `get_mut` does not: a map with `String` keys can be searched with a `&str`, so `counts.get_mut(buf.as_str())` finds a known name with no allocation. Only for a new name does `insert(buf.clone(), 1)` copy the buffer into a key the map keeps.

```rust
let mut buf = String::new();
for player in ["steve", "alex", "steve"] {
    buf.clear();                                   // empty, but keeps its room
    buf.push_str("player:");
    buf.extend(player.chars().flat_map(char::to_uppercase));
    match deaths.get_mut(buf.as_str()) {           // "player:STEVE", no new String
        Some(count) => *count += 1,
        None => {
            deaths.insert(buf.clone(), 1);        // copy only for a newcomer
        }
    }
}
```

## Your turn

- Before the loop, make one empty `String` buffer.
- For each name: clear the buffer, fill it with the name's lowercased characters, then add 1 to its count with `get_mut`, or `insert` a clone of the buffer with 1 for a new name.
