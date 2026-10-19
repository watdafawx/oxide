Hopper has to open a database written by last month's version and still work. That takes two tools: **transactions** and **migrations**.

A **transaction** makes several statements one unit: all of them land, or none. It is also much faster than autocommitting every insert.

```rust
let tx = conn.transaction()?;      // needs `&mut Connection`
tx.execute("INSERT INTO t (x) VALUES (1)", [])?;
tx.execute("INSERT INTO t (x) VALUES (2)", [])?;
tx.commit()?;                      // drop without commit = rollback
```

A **prepared statement** parses the SQL once and runs it many times: `let mut stmt = tx.prepare(sql)?;` then `stmt.execute(params![..])?` in a loop.

A **migration** brings an old schema up to date. SQLite gives every database a free integer, `PRAGMA user_version`. Read it, run the steps above that number, then write the new number, all inside one transaction so a failed upgrade leaves the file untouched.

```rust
let v: i64 = conn.pragma_query_value(None, "user_version", |r| r.get(0))?;
if v < 1 { /* create tables */ }
if v < 2 { /* ALTER TABLE ... */ }
conn.pragma_update(None, "user_version", 2)?;
```

## Your turn

In `src/lib.rs` (`version` is done for you):

- `migrate`: version 1 creates `notes (id, body)` with a `CHECK (body <> '')`; version 2 adds `pinned INTEGER NOT NULL DEFAULT 0`. Safe to run any number of times.
- `add_many`: insert all bodies in one transaction with one prepared statement. Return how many were added.
