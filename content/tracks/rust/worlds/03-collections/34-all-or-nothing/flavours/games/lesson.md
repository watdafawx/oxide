A kit file sets the counts for a player's hotbar: `1, 16, x, 8`. Skipping the bad one, as `filter_map` did, would be wrong here: a kit with a typo should be fixed by the server owner, not handed out half-filled. The plugin wants every number, or the first problem and nothing else.

**A list of Results.** `fields.iter().map(|f| parse_count(f))` gives one `Result` per field: `Ok(1)`, `Ok(16)`, `Err("bad count 'x'")`, `Ok(8)`. Collecting that into a `Vec<Result<u32, String>>` works, but then every caller has to dig through it.

**One Result instead.** Ask `collect` for a `Result<Vec<u32>, String>` and it changes what it builds: if every item is `Ok`, you get `Ok` of all the values; at the first `Err`, it stops and gives back that error. Because iterators are lazy, nothing after the bad field is even parsed.

```rust
let levels: Result<Vec<u32>, String> = ["30", "41"].iter().map(|l| parse_level(l)).collect();
// Ok([30, 41]); with "30", "max", "41" it would be the error for "max"
```

**Say where.** With many fields, the message should say which one failed. `enumerate` gives the position, and a `map_err` inside the `map` puts it in front of the message: `slot 3: bad count 'x'`, counting from 1 for players.

**Options too.** The same works for `Option`: collecting `Option`s into an `Option<Vec<_>>` gives `Some` of every value, or `None` as soon as one is missing.

## Your turn

- In `parse_all`, map every field through `parse_count` and collect into the `Result`.
- In `parse_all_numbered`, enumerate first, and `map_err` each error to `slot N: <error>`.
