Oxide's progress lives in a `quests` table, so the first thing the engine does on start is make sure that table exists. Rust talks to SQLite through the `rusqlite` crate, and every conversation starts with a connection.

**A connection.** `Connection::open_in_memory()` makes a fresh database that lives in the computer's memory and is gone when the program ends, which is exactly what a test wants. `Connection::open("progress.db")` is the same thing backed by a file. Either way you get a **connection**: the handle you send SQL through.

**Send it SQL.** `conn.execute(sql, [])` runs one statement that gives back no rows, such as `CREATE TABLE`. The SQL is plain text in a Rust string. The `[]` is the list of values to fill into the statement; this one has none, and the next quest fills it.

**Each step can fail.** The SQL may have a typo, or the file may be locked. So both calls return `rusqlite::Result<T>`, which is a normal `Result` whose error is a `rusqlite::Error`. `use rusqlite::Result;` lets you write it short, and `?` from world 03 passes any error up to the caller.

```rust
use rusqlite::{Connection, Result};

fn open_log() -> Result<Connection> {
    let conn = Connection::open_in_memory()?;
    conn.execute("CREATE TABLE streaks (day TEXT NOT NULL, minutes INTEGER NOT NULL)", [])?;
    Ok(conn)
}
```

**Hand it back.** A function that made the connection returns it inside `Ok(..)`, and the caller keeps using it. When the last owner drops it, the connection closes; an in-memory database disappears with it.

**First build.** This quest downloads SQLite's own source and compiles it (the `bundled` feature), so the first run takes one to three minutes and needs a C compiler. After that, runs take seconds.

## Your turn

- In `open`, open a database in memory.
- Run the `CREATE TABLE quests ...` statement from the doc comment on it.
- Hand the connection back.
