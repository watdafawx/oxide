The ESL asset checker compares the names on a sheet with the photo files in a folder and spots likely renames. Before a show that is the difference between a blank player card and a fixed file name.

**Same name, two spellings.** `normalize_name` lowercases, drops common accents and keeps letters and digits, so `Zoë`, `zoe.png` and `ZOE` all meet.

**Close is a rename.** A file that matches nobody but is one or two letters off a missing player is probably theirs: `Mos.png` for Moss. The Levenshtein distance counts single-letter inserts, deletes and changes.

**Row by row.** The classic table has a row per letter of `a`; each row needs only the one before. A cell is the cheapest of three moves:

```rust
row.push((previous[j] + cost).min(previous[j + 1] + 1).min(row[j] + 1));
```

**Chars, not bytes.** Compare `chars()`, so an accented letter counts as one edit, not two.

## Your turn

- Write `levenshtein` with one previous row and one new row per letter of `a`.
- Save. `compare_names` uses it for rename suggestions.
