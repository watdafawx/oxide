The quest list shows every quest, and the "best rewards" panel shows titles by XP, biggest first. Those are `SELECT`s with many rows, and `query_row` only takes one. This quest reads them all into a `Vec`.

**Prepare the statement.** `conn.prepare(sql)?` hands SQLite the SQL once and gives back a **statement**, the SQL checked and ready to run. It goes in a `let mut stmt`, because running it changes its inner state.

**Map every row.** `stmt.query_map(params, |row| ...)` runs the statement and calls your closure once per row, the same closure shape as `query_row`. To fill a struct, read each column with `row.get(n)?` and wrap the result in `Ok(..)`: the `?` is there because any column can fail to convert. What comes back is not a `Vec` yet but an iterator with one `Result` per row.

**Collect them all.** From world 03: `.collect()` on an iterator of `Result`s, into a `Result<Vec<_>>`, gives the whole list or the first error. Your function's return type tells `collect` what to build, so the last line can be just `.collect()`.

```rust
let mut stmt = conn.prepare("SELECT day, minutes FROM streaks ORDER BY day")?;
let days: Vec<Streak> = stmt
    .query_map([], |row| Ok(Streak { day: row.get(0)?, minutes: row.get(1)? }))?
    .collect::<Result<_>>()?;
```

**Order is not free.** Without `ORDER BY`, SQLite may return rows in any order it likes, and it can change when the table grows. If the order matters to the screen, say it in the SQL: `ORDER BY id` for oldest first, `ORDER BY xp DESC` for biggest first.

## Your turn

- In `list_quests`, prepare `SELECT id, title, xp FROM quests ORDER BY id`.
- Map each row into a `QuestRow` and collect the rows.
- In `biggest_first`, do the same with only the titles, sorted by xp, biggest first.
