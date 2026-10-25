The launcher opens one mod with `GET /mods/7`, and uninstalls one with `DELETE /mods/7`. Either can name an id that doesn't exist, and the launcher needs to hear `404 Not Found`, not `500`: "no such mod" is a normal answer, a broken database is not.

**Two kinds of nothing.** From quest 04, `query_row` returns an `Err` when no row matches. But it also returns an `Err` when the database is broken. With `.map_err(internal)?` both would become a 500, and the launcher would think the server crashed every time someone opened an uninstalled mod.

**Make "none" a value.** `use rusqlite::OptionalExtension;` adds one method to `query_row`'s result: `.optional()` turns "no row" into `Ok(None)`, a row into `Ok(Some(row))`, and keeps every real error as `Err`. Bringing a trait into scope to gain a method is the same trick as `Context` in world 04.

**None is 404.** After `.optional().map_err(internal)?` you hold an `Option<ModRow>`, and a `match` from world 01 makes the answer: `Some(m)` becomes `Ok(Json(m))`, `None` becomes `Err(StatusCode::NOT_FOUND)`. A real database error has already left as a 500 through the `?`.

```rust
let found = conn
    .query_row("SELECT seed FROM worlds WHERE name = ?1", params![name], |r| r.get(0))
    .optional()
    .map_err(internal)?;
match found {
    Some(seed) => Ok(Json(Seed { seed })),
    None => Err(StatusCode::NOT_FOUND),
}
```

**Delete counts rows.** `conn.execute(..)` hands back how many rows it changed. A `DELETE` that changed 0 rows found nothing, so it answers 404; otherwise it answers `204 No Content`: done, and there is nothing to send back. Deleting the same id twice therefore gives 204, then 404.

**For the curious.** `found.map(Json).ok_or(StatusCode::NOT_FOUND)` says the same as the `match` in one line: `map` changes the value inside a `Some`, and `ok_or` turns an `Option` into a `Result`.

## Your turn

- In `get_mod`, read the mod with this id as an `Option`, and answer it or 404.
- In `remove_mod`, delete the mod with this id and answer 204, or 404 when nothing was deleted.
