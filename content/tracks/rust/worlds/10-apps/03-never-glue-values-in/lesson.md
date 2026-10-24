Next the engine adds quests to the table. The obvious way is to build the `INSERT` text with `format!`, and it works for "Variables". Then a quest called "Rust's traits" arrives, and the insert fails.

**Glue it in.** `format!("INSERT INTO quests (title, xp) VALUES ('{title}', {xp})")` turns into `VALUES ('Variables', 40)`, which is valid SQL. The starter does exactly this, and it looks fine.

**An apostrophe.** With "Rust's traits" the text becomes `VALUES ('Rust's traits', 60)`. In SQL a `'` ends a piece of text, so SQLite reads `'Rust'`, then a stray `s traits'` it cannot make sense of, and refuses the whole statement:

```text
SqlInputError { error: Error { code: Unknown, extended_code: 1 }, msg: "near \"s\": syntax error",
    sql: "INSERT INTO quests (title, xp) VALUES ('Rust's traits', 60)", offset: 45 }
```

**Worse on purpose.** Now picture a title typed by someone who knows this: `x', 0); DELETE FROM quests; --`. Glued in, that is your `INSERT` followed by their `DELETE`. The database cannot tell your SQL from theirs. (rusqlite's `execute` happens to refuse a second statement, but a glued-in value can still bend the one you wrote.) This is called **SQL injection**, and it is one of the oldest and most common security holes on the web.

**Send values separately.** The fix is to never put a value inside the SQL text. Write a **placeholder**, `?1`, `?2`, where each value goes (no quotes around it), and pass the values next to the statement with the `params!` macro. SQLite receives the statement and the values separately, so a value is always just a value, whatever letters it contains.

```rust
use rusqlite::params;

conn.execute(
    "INSERT INTO streaks (day, minutes) VALUES (?1, ?2)",
    params![day, minutes],
)?;
let id = conn.last_insert_rowid();
```

**The new id.** After an `INSERT`, `conn.last_insert_rowid()` is the id the database just handed the new row: 1 for the first, then 2, 3. It is how the engine knows which row it made.

## Your turn

- In `add_quest`, replace the glued-in SQL with one that has `?1` and `?2`.
- Pass `title` and `xp` with `params!` (import it from `rusqlite`).
- Keep returning the new row's id.
