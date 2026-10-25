The quest screen needs a title for quest 7, and the home screen says "12 quests worth 50 XP or more". Each is a question with exactly one answer, and `query_row` asks it.

**One row, one answer.** `conn.query_row(sql, params, f)` runs a `SELECT` that should give back exactly one row, and hands that row to `f`, a closure you write. Whatever the closure returns is what `query_row` returns, inside a `Result`.

**Read a column.** In the closure, `row.get(0)` reads the first column you selected, `row.get(1)` the second: columns count from 0, like a `Vec`. It converts the SQL value into the Rust type the function returns, so a `TEXT` column read into a `String` just works. Here the type comes from your return type, `Result<String>`.

**Let SQL do the counting.** `SELECT COUNT(*) FROM quests WHERE ...` makes SQLite count the matching rows and give back one row holding one number. You don't load every quest into Rust to count them; the database is good at this.

```rust
let minutes: i64 = conn.query_row(
    "SELECT minutes FROM streaks WHERE day = ?1",
    params![day],
    |row| row.get(0),
)?;
```

**No row is an error.** If the `WHERE` matches nothing, there is no row to hand the closure, so `query_row` returns an `Err` (`QueryReturnedNoRows`). For `title_of(99)` that is the right answer for now; a later quest turns it into a clean "not found".

## Your turn

- In `title_of`, select the `title` of the quest whose id is `?1`.
- In `count_worth_at_least`, select `COUNT(*)` of the quests whose xp is at least `?1`.
- Both are one `query_row` with `row.get(0)` in the closure.
