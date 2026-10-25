The mod screen needs the name of mod 7, and the header says "212 mods enabled". Each is a question with exactly one answer, and `query_row` asks it.

**One row, one answer.** `conn.query_row(sql, params, f)` runs a `SELECT` that should give back exactly one row, and hands that row to `f`, a closure you write. Whatever the closure returns is what `query_row` returns, inside a `Result`.

**Read a column.** In the closure, `row.get(0)` reads the first column you selected, `row.get(1)` the second: columns count from 0, like a `Vec`. It converts the SQL value into the Rust type the function returns, so a `TEXT` column read into a `String` just works. Here the type comes from your return type, `Result<String>`.

**Let SQL do the counting.** `SELECT COUNT(*) FROM mods WHERE ...` makes SQLite count the matching rows and give back one row holding one number. You don't load three hundred mods into Rust to count them; the database is good at this.

```rust
let seed: i64 = conn.query_row(
    "SELECT seed FROM worlds WHERE name = ?1",
    params![name],
    |row| row.get(0),
)?;
```

**No row is an error.** If the `WHERE` matches nothing, there is no row to hand the closure, so `query_row` returns an `Err` (`QueryReturnedNoRows`). For `name_of(99)` that is the right answer for now; a later quest turns it into a clean "not found".

## Your turn

- In `name_of`, select the `name` of the mod whose id is `?1`.
- In `count_by_state`, select `COUNT(*)` of the mods whose `enabled` is `?1`.
- Both are one `query_row` with `row.get(0)` in the closure.
