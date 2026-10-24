When Oxide installs a new world it adds twenty quests at once. If quest 8 has a blank title and the insert fails, inserting one by one leaves seven quests in the table, and the app now thinks the world is seven quests long. You want the whole world, or none of it.

**The word for it.** A **transaction** groups several statements into one unit: either all of them land, or none do. Databases guarantee it even if the program crashes halfway, which is why every bank and every save file uses them.

**Start, then commit.** `let tx = conn.transaction()?;` starts one. You run statements on `tx` exactly as on a connection, and nobody else sees them yet. `tx.commit()?` makes them all land at once.

**Dropping undoes.** If you never reach `commit`, for example because `?` returned early on an error, `tx` is dropped at the end of the function, and dropping a transaction **rolls it back**: every statement in it is undone. The `Drop` you met in world 06 is doing the cleanup for you.

```rust
let tx = conn.transaction()?;
for (day, minutes) in days {
    tx.execute("INSERT INTO streaks (day, minutes) VALUES (?1, ?2)", params![day, minutes])?;
}
tx.commit()?;
```

**Why `&mut`.** `transaction()` needs `&mut Connection`, which means no other code can use the connection until the transaction is done. That is the borrow checker making sure nobody slips a statement in the middle.

**Faster, too.** Outside a transaction SQLite saves to disk after every single insert. Inside one it saves once at `commit`, so a thousand inserts go from seconds to milliseconds. For the curious: `tx.prepare(sql)` once, then `stmt.execute(..)` in the loop, saves parsing the SQL a thousand times.

## Your turn

- In `add_world`, start a transaction on `conn`.
- Insert every `(title, xp)` pair on the transaction.
- Commit, then return how many quests you added.
