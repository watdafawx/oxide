The quest screen opens one quest with `GET /quests/7`, and the editor removes one with `DELETE /quests/7`. Either can name an id that doesn't exist, and the screen needs to hear `404 Not Found`, not `500`: "no such quest" is a normal answer, a broken database is not.

**Two kinds of nothing.** From quest 04, `query_row` returns an `Err` when no row matches. But it also returns an `Err` when the database is broken. With `.map_err(internal)?` both would become a 500, and the screen would think the server crashed every time someone opened a deleted quest.

**Make "none" a value.** `use rusqlite::OptionalExtension;` adds one method to `query_row`'s result: `.optional()` turns "no row" into `Ok(None)`, a row into `Ok(Some(row))`, and keeps every real error as `Err`. Bringing a trait into scope to gain a method is the same trick as `Context` in world 04.

**None is 404.** After `.optional().map_err(internal)?` you hold an `Option<Quest>`, and a `match` from world 01 makes the answer: `Some(q)` becomes `Ok(Json(q))`, `None` becomes `Err(StatusCode::NOT_FOUND)`. A real database error has already left as a 500 through the `?`.

```rust
let found = conn
    .query_row("SELECT minutes FROM streaks WHERE day = ?1", params![day], |r| r.get(0))
    .optional()
    .map_err(internal)?;
match found {
    Some(minutes) => Ok(Json(Minutes { minutes })),
    None => Err(StatusCode::NOT_FOUND),
}
```

**Delete counts rows.** `conn.execute(..)` hands back how many rows it changed. A `DELETE` that changed 0 rows found nothing, so it answers 404; otherwise it answers `204 No Content`: done, and there is nothing to send back. Deleting the same id twice therefore gives 204, then 404.

**For the curious.** `found.map(Json).ok_or(StatusCode::NOT_FOUND)` says the same as the `match` in one line: `map` changes the value inside a `Some`, and `ok_or` turns an `Option` into a `Result`.

## Your turn

- In `get_one`, read the quest with this id as an `Option`, and answer it or 404.
- In `remove`, delete the quest with this id and answer 204, or 404 when nothing was deleted.
