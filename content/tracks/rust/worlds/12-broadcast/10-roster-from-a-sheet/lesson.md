The sheet parser's best idea is its "sheet types": a file that says which column header fills which field, so nothing breaks when a producer adds a column in the middle. Rosters here come the same way, as a CSV export.

**Headers first.** `csv::Reader` reads the first row as headers. Clone them: the reader is borrowed again for the rows.

**Find by name.** A small closure finds each header's position, or an error that names the missing column:

```rust
let column = |name: &str| headers.iter().position(|h| h.trim() == name).ok_or_else(|| format!("missing column \"{name}\""));
```

**Check before reading.** Look up all three columns with `?` before the first row, so a wrong sheet fails at once with a clear message.

**Messy cells.** Sheets have stray spaces and empty rows. Trim every cell, and skip rows without a nickname.

## Your turn

- In `read_roster`, find the `NICKNAME`, `TEAM` and `PHOTO` columns by header.
- Read each row into a `Player`, trimmed, skipping rows without a nickname. Save.
