The mod list shows every mod, and the search panel shows names from A to Z. Those are `SELECT`s with many rows, and `query_row` only takes one. This quest reads them all into a `Vec`.

**Prepare the statement.** `conn.prepare(sql)?` hands SQLite the SQL once and gives back a **statement**, the SQL checked and ready to run. It goes in a `let mut stmt`, because running it changes its inner state.

**Map every row.** `stmt.query_map(params, |row| ...)` runs the statement and calls your closure once per row, the same closure shape as `query_row`. To fill a struct, read each column with `row.get(n)?` and wrap the result in `Ok(..)`: the `?` is there because any column can fail to convert. What comes back is not a `Vec` yet but an iterator with one `Result` per row.

**Collect them all.** From world 03: `.collect()` on an iterator of `Result`s, into a `Result<Vec<_>>`, gives the whole list or the first error. Your function's return type tells `collect` what to build, so the last line can be just `.collect()`.

```rust
let mut stmt = conn.prepare("SELECT name, seed FROM worlds ORDER BY seed")?;
let worlds: Vec<World> = stmt
    .query_map([], |row| Ok(World { name: row.get(0)?, seed: row.get(1)? }))?
    .collect::<Result<_>>()?;
```

**Order is not free.** Without `ORDER BY`, SQLite may return rows in any order it likes, and it can change when the table grows. If the order matters to the screen, say it in the SQL: `ORDER BY id` for oldest first, `ORDER BY name` for A to Z.

## Your turn

- In `list_mods`, prepare `SELECT id, name, enabled FROM mods ORDER BY id`.
- Map each row into an `InstalledMod` and collect the rows.
- In `a_to_z`, do the same with only the names, sorted by name.
