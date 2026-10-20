**Why now:** this joins the mod database to the mod API: handlers on top that add, list and delete mods in SQLite underneath.

The only new problem is **sharing one connection** between handlers that may run on different threads.

- `rusqlite::Connection` is `Send` but not `Sync`, so put it behind a mutex: `Arc<Mutex<Connection>>`.
- `State<Db>` clones the `Arc` into each handler; `db.lock().unwrap()` gives you the connection for as long as the guard lives.
- Never hold that guard across an `.await`. These handlers have none, so it is fine.
- `.optional()` (from `rusqlite::OptionalExtension`) turns "no row" into `Ok(None)` instead of an error, perfect for a 404.
- Unexpected database errors become `500` through `fn internal(e: rusqlite::Error) -> StatusCode`.

```rust
async fn one(State(db): State<Db>, Path(id): Path<i64>) -> Result<Json<Note>, StatusCode> {
    let conn = db.lock().unwrap();
    let note = conn
        .query_row("SELECT id, body FROM notes WHERE id = ?1", params![id], |r| {
            Ok(Note { id: r.get(0)?, body: r.get(1)? })
        })
        .optional()
        .map_err(internal)?;
    note.map(Json).ok_or(StatusCode::NOT_FOUND)
}
```

Three more reminders from the rusqlite quest:

- `conn.execute(..)` returns how many rows it changed. A `DELETE` that returns `0` found nothing.
- A list comes from `conn.prepare(sql)`, then `stmt.query_map([], |r| Ok(Note { .. }))`, then **`.collect::<Result<Vec<_>, _>>()`**; each of those steps ends in `.map_err(internal)?`.
- `conn.last_insert_rowid()` is the id the INSERT just made.

Routing and the table are set up for you; the tests use a fresh in-memory database each time.

## Your turn

In `src/lib.rs`, write the four handlers:

- `create`: INSERT, answer `201 Created` with the `Note`.
- `list`: all notes ordered by id.
- `get_one`: the note, or `404`.
- `remove`: `204 No Content`, or `404` when nothing was deleted.
