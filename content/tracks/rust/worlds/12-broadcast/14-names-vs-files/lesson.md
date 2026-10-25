The ESL asset checker compares the names on the roster with the photo files in the folder. Before a show, catching `Mos.png` for the player Moss is the difference between a blank player card and a one-second rename. The table from the last quest becomes `levenshtein`, and `compare_names` (already written) uses it.

**Same name, two spellings.** `normalize_name` (already written) lowercases, drops accents and keeps letters and digits, so `Zoë`, `zoe.png` and `ZOE` all become `zoe`. A file at distance 2 or less from a missing player is suggested as theirs.

**Chars, not bytes.** Walk the names with `.chars()`, so `ë` counts as one letter, not two bytes. Collect `b`'s letters into a `Vec<char>` first: you walk them once for every letter of `a`, and the length is the number of columns.

**The first row.** `previous` starts as 0, 1, 2, ... up to the length of `b`. `(0..=n).collect()` makes that list: `..=` includes the end, so there are `n + 1` cells, one per column plus the empty start.

**The same two rows, a simpler rule.** Here is the two-row loop with an easier rule, where each new cell is the sum of the two cells above it. Look at the moving parts: a new `row` that starts with its first cell, one `push` per cell, then `previous = row`:

```rust
let mut previous = vec![1, 1];
for _ in 0..3 {
    let mut row = vec![1];
    for j in 0..previous.len() - 1 {
        row.push(previous[j] + previous[j + 1]);
    }
    row.push(1);
    previous = row;
}
// previous is now [1, 4, 6, 4, 1]
```

**Where the neighbours are.** For the letter of `a` at position `i`, the new row starts with `i + 1`, the left column. While you build the cell after `row[j]`: up-left is `previous[j]`, above is `previous[j + 1]`, and left is `row[j]`, the cell you pushed last. `x.min(y)` is the smaller number, and it chains: `x.min(y).min(z)`.

**A bool as a number.** The swap costs 0 when the letters match and 1 when they differ. `usize::from(ca != cb)` turns `false` into 0 and `true` into 1, with no `if`.

**The answer.** After the last letter of `a`, the last cell of `previous` is the bottom-right of the table.

## Your turn

- In `levenshtein`, collect `b`'s letters and start `previous` at 0, 1, 2, ...
- For each letter of `a`, build a `row` from the three neighbours of each cell, then make it `previous`.
- Give back the last cell of `previous`. Save.
