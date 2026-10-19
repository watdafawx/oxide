**Why now:** a mod list in a text file works for ten mods. For 300 you want to ask "which are disabled?" or "which are older than 1.20?" without loading everything. That is a **database**, and SQLite is one in a single file (or in memory for tests) with no server to run.

```rust
use rusqlite::{params, Connection};

let conn = Connection::open_in_memory()?;
conn.execute(
    "CREATE TABLE mods (id INTEGER PRIMARY KEY, name TEXT NOT NULL, enabled INTEGER NOT NULL)",
    [],
)?;

conn.execute("INSERT INTO mods (name, enabled) VALUES (?1, 1)", params!["sodium"])?;
let id = conn.last_insert_rowid();                      // 1

let mut stmt = conn.prepare("SELECT id, name FROM mods ORDER BY id")?;
let names: Vec<String> = stmt
    .query_map([], |row| row.get(1))?
    .collect::<Result<_, _>>()?;
```

- `?1`, `?2` are **parameters**. Pass values through `params![...]`, never by gluing them into the SQL string.
- `INTEGER PRIMARY KEY` ids are handed out automatically: 1, 2, 3.
- `row.get(0)?` reads a column by position. It converts to the Rust type you ask for (`i64`, `String`, `bool`).
- `rusqlite` is a synchronous crate with SQLite bundled: no install needed.

## Your turn

In `src/lib.rs`:

- `open_mod_db`: in-memory database with the `mods` table (given).
- `add_mod`: insert an enabled mod, return its id.
- `list_mods`: every row as an `InstalledMod`, ordered by id.
