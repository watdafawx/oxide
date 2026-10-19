**Why now:** you shipped the launcher with a `mods` table. Next release you want a `favourite` flag, and users already have databases full of mods that must keep working. A **migration** upgrades old databases step by step. And installing a whole modpack must be **all or nothing**: half a pack is worse than none. That is a **transaction**.

```rust
let tx = conn.transaction()?;               // BEGIN
tx.execute("INSERT INTO mods (name) VALUES (?1)", params!["sodium"])?;
tx.execute("INSERT INTO mods (name) VALUES (?1)", params![""])?;  // fails: CHECK
tx.commit()?;                               // never reached: dropping `tx` rolls back

// schema version lives in SQLite itself
let v: i64 = conn.pragma_query_value(None, "user_version", |r| r.get(0))?;
```

- A transaction groups statements. `commit()` keeps them. Dropping `tx` without committing **rolls everything back**.
- `PRAGMA user_version` is a free integer inside the database file. Use it as the schema version.
- A migration step is `if v < N { change schema }`. A fresh database (version 0) runs every step, an old one only the missing ones. Run the whole thing in one transaction.
- `CHECK (name <> '')` makes the database itself refuse blank names. Great for testing rollback.

## Your turn

In `src/lib.rs`:

- `migrate`: v1 creates `mods`, v2 adds the `favourite` column; set `user_version` to 2; one transaction.
- `add_mods`: insert every name in a single transaction; any failure keeps nothing.
