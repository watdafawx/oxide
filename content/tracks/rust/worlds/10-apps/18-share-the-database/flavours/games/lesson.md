This is where the two halves meet: the launcher calls `POST /mods` and `GET /mods`, and the mods live in the SQLite table from the start of this world. Every handler needs the same connection, and requests can arrive on several threads at once.

**One connection, one user at a time.** A `rusqlite::Connection` can move to another thread, but two threads must not use it at the same moment (in world 07's words, it is `Send` but not `Sync`). axum handles requests on several threads, so handing every handler a plain shared connection does not compile.

**Lock it.** The world 07 answer: `Arc<Mutex<Connection>>`. The `Arc` gives every request a handle to the same connection, and the `Mutex` lets one request in at a time. `let conn = db.lock().unwrap();` waits for your turn and gives you the connection; the lock is released when `conn` is dropped at the end of the handler. The starter calls this type `Db`.

**Hold it briefly.** While you hold the lock, every other request waits. These handlers lock, run their SQL and finish, which is fine. Never keep a `std::sync::Mutex` guard across an `.await`: the waiting would block a whole worker thread (world 08). These handlers have no `.await` inside, so there is nothing to worry about.

**Database errors are 500.** If SQLite fails, that is the server's fault, not the client's. `fn internal(_: rusqlite::Error) -> StatusCode` in the starter turns any rusqlite error into `500 Internal Server Error`, and `.map_err(internal)?` from world 03 does it in one step after every call.

```rust
async fn add_world(State(db): State<Db>, Json(world): Json<NewWorld>)
    -> Result<StatusCode, StatusCode> {
    let conn = db.lock().unwrap();
    conn.execute(
        "INSERT INTO worlds (name, seed) VALUES (?1, ?2)",
        params![world.name, world.seed],
    )
    .map_err(internal)?;
    Ok(StatusCode::CREATED)
}
```

**The list.** `GET /mods` is quest 05 again, inside a handler: `prepare`, `query_map`, then `.collect::<Result<Vec<_>, _>>()`, with `.map_err(internal)?` after each step, and the `Vec` wrapped in `Json`. A `Vec` of structs becomes a JSON array of objects.

## Your turn

- In `add_mod`, lock the database, insert the mod, and answer `201 Created` with the stored `ModRow`.
- In `list_mods`, lock the database and answer every mod, ordered by id, as JSON.
- After each rusqlite call, map its error to a 500 with `internal`.
