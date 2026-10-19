**Why now:** the mod database starts as one table of mods, and `rusqlite` is how Rust creates it, fills it and reads it back.

**rusqlite** is the crate that talks to SQLite, and the whole API fits on one screen.

- `Connection::open_in_memory()` makes a database that lives in RAM. Perfect for tests; `Connection::open(path)` is the file version.
- `conn.execute(sql, params)` runs a statement that returns no rows and gives back how many rows it changed.
- `?1`, `?2` in the SQL are placeholders. Fill them with `params![a, b]`. **Never** format values into the SQL string.
- `conn.prepare(sql)` then `stmt.query_map(params, |row| ...)` runs a query and maps each row to your type. `row.get(0)?` reads column 0, and the type is inferred from where you put it.

```rust
use rusqlite::{params, Connection};

let conn = Connection::open_in_memory()?;
conn.execute("CREATE TABLE mods (id INTEGER PRIMARY KEY, name TEXT NOT NULL)", [])?;
conn.execute("INSERT INTO mods (name) VALUES (?1)", params!["sodium"])?;

let id = conn.last_insert_rowid();
let name: String = conn.query_row("SELECT name FROM mods WHERE id = ?1", params![id], |r| r.get(0))?;
```

SQLite has no bool type: store `0` / `1`. rusqlite converts `bool` for you.

**First build:** this quest downloads and compiles SQLite itself (the `bundled` feature builds C code, so it needs a C compiler, about 1-3 min the first time). Later runs take seconds.

## Your turn

In `src/lib.rs` (`open` is given: it makes the table):

- `add_mod`: insert an enabled mod, return `conn.last_insert_rowid()`.
- `list_mods`: `query_map` every row into a `Mod`, ordered by id.
