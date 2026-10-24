Rosters come from a Google Sheet the producers keep, exported as CSV. ESL's sheet parser had one great idea for those: find each column by its header, not by its position, so nothing breaks when a producer adds a column in the middle. The HUD reads its roster the same way.

**Headers first.** `csv::Reader::from_reader(text.as_bytes())` reads CSV from the text, and treats the first row as headers: `reader.headers()` gives them (or a read error). Clone them, because the reader is borrowed again for the rows.

**Find by name.** `headers.iter().position(|h| h.trim() == name)` gives the column's index, or `None` when no header matches; `ok_or_else` turns that `None` into an error message. For one column:

```rust
let country = headers.iter().position(|h| h.trim() == "Country").ok_or_else(|| "missing column \"Country\"".to_string())?;
```

**Three columns, one rule.** You need three columns, so put the lookup in a closure that takes the header name, `let column = |name: &str| ..;`, and call it three times. The error then names whichever column is missing.

**Check before reading.** Look up all three columns with `?` before the first row, so a wrong sheet fails at once with a clear message.

**Messy cells.** `for record in reader.records()` gives each row as a `Result`. Sheets have stray spaces and empty rows: `record.get(i)` gives `Option<&str>` for a cell; `unwrap_or("")` makes a missing cell empty. Trim every cell, and `continue` past rows without a nickname.

## Your turn

- In `read_roster`, find the `NICKNAME`, `TEAM` and `PHOTO` columns by header.
- Read each row into a `Player`, trimmed, skipping rows without a nickname. Save.
