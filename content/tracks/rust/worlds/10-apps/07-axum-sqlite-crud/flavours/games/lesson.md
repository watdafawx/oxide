**Why now:** this is the mod database as other programs see it: one HTTP API in front of SQLite. Adding a mod, listing mods, fetching one and removing it are the four verbs (**CRUD**) every app backend ends up with.

```rust
type Db = Arc<Mutex<Connection>>;                      // one connection, shared and locked

async fn add(State(db): State<Db>, Json(new): Json<NewMod>)
    -> Result<(StatusCode, Json<ModRow>), StatusCode> {
    let conn = db.lock().unwrap();
    conn.execute("INSERT INTO mods (name) VALUES (?1)", params![new.name])
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::CREATED, Json(ModRow { id: conn.last_insert_rowid(), name: new.name })))
}
```

```rust
// one row or nothing: `.optional()` (use rusqlite::OptionalExtension) makes "no row" an Ok(None)
let found: Option<ModRow> = conn
    .query_row("SELECT id, name FROM mods WHERE id = ?1", params![id], |r| {
        Ok(ModRow { id: r.get(0)?, name: r.get(1)? })
    })
    .optional()
    .map_err(internal)?;                       // `internal` is given: any db error -> 500
found.map(Json).ok_or(StatusCode::NOT_FOUND)   // Some(row) -> Json(row), None -> 404

// a list: prepare, query_map, then collect the Results into one Result<Vec<_>, _>
let rows = stmt.query_map([], |r| Ok(ModRow { id: r.get(0)?, name: r.get(1)? }))
    .map_err(internal)?
    .collect::<Result<Vec<_>, _>>()
    .map_err(internal)?;
```

- Routes: `/mods` takes `get(list).post(add)`. `/mods/{id}` takes `get(one).delete(remove)`.
- A `rusqlite::Connection` is not `Sync`, so share it as `Arc<Mutex<Connection>>`. Lock, query, drop the guard. Never hold it across an `.await`.
- `query_row(..).optional()` turns "no row" into `Ok(None)`, which is how you spot a 404.
- `execute` returns how many rows changed: perfect to tell a real delete (204) from a missing one (404).
- Map unexpected database errors to `500`. Expected cases (missing id) get their own status.

## Your turn

In `src/lib.rs` fill the four handlers (`add_mod`, `list_mods`, `get_mod`, `remove_mod`). `open_db`, the types and `app` are given.
