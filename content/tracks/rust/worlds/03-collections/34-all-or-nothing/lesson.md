A delivery note lists quantities: `12, 6, x, 1`. Skipping the bad one, as `filter_map` did, would be wrong here: a delivery with a typo must be checked by a person, not half-booked. The tool wants every number, or the first problem and nothing else.

**A list of Results.** `fields.iter().map(|f| parse_qty(f))` gives one `Result` per field: `Ok(12)`, `Ok(6)`, `Err("bad qty 'x'")`, `Ok(1)`. Collecting that into a `Vec<Result<u32, String>>` works, but then every caller has to dig through it.

**One Result instead.** Ask `collect` for a `Result<Vec<u32>, String>` and it changes what it builds: if every item is `Ok`, you get `Ok` of all the values; at the first `Err`, it stops and gives back that error. Because iterators are lazy, nothing after the bad field is even parsed.

```rust
let hours: Result<Vec<u32>, String> = ["8", "6"].iter().map(|h| parse_hours(h)).collect();
// Ok([8, 6]); with "8", "lots", "6" it would be the error for "lots"
```

**Say where.** With many fields, the message should say which one failed. `enumerate` gives the position, and a `map_err` inside the `map` puts it in front of the message: `item 3: bad qty 'x'`, counting from 1 for people.

**Options too.** The same works for `Option`: collecting `Option`s into an `Option<Vec<_>>` gives `Some` of every value, or `None` as soon as one is missing.

## Your turn

- In `parse_all`, map every field through `parse_qty` and collect into the `Result`.
- In `parse_all_numbered`, enumerate first, and `map_err` each error to `item N: <error>`.
