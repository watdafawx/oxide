Installing a modpack adds forty mods at once. If mod 8 has a blank name and the insert fails, inserting one by one leaves seven mods in the table: half a pack, which crashes the game worse than no pack at all. You want the whole pack, or none of it.

**The word for it.** A **transaction** groups several statements into one unit: either all of them land, or none do. Databases guarantee it even if the program crashes halfway, which is why every bank and every save file uses them.

**Start, then commit.** `let tx = conn.transaction()?;` starts one. You run statements on `tx` exactly as on a connection, and nobody else sees them yet. `tx.commit()?` makes them all land at once.

**Dropping undoes.** If you never reach `commit`, for example because `?` returned early on an error, `tx` is dropped at the end of the function, and dropping a transaction **rolls it back**: every statement in it is undone. The `Drop` you met in world 06 is doing the cleanup for you.

```rust
let tx = conn.transaction()?;
for (name, seed) in worlds {
    tx.execute("INSERT INTO worlds (name, seed) VALUES (?1, ?2)", params![name, seed])?;
}
tx.commit()?;
```

**Why `&mut`.** `transaction()` needs `&mut Connection`, which means no other code can use the connection until the transaction is done. That is the borrow checker making sure nobody slips a statement in the middle.

**Faster, too.** Outside a transaction SQLite saves to disk after every single insert. Inside one it saves once at `commit`, so a thousand inserts go from seconds to milliseconds. For the curious: `tx.prepare(sql)` once, then `stmt.execute(..)` in the loop, saves parsing the SQL a thousand times.

## Your turn

- In `install_pack`, start a transaction on `conn`.
- Insert every name on the transaction, enabled.
- Commit, then return how many mods you added.
