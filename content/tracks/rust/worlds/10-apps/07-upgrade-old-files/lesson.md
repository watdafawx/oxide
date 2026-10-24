Version 1 of the engine shipped with a `quests` table of id, title and xp. Version 2 also remembers whether each quest is done. Thousands of people already have a version-1 file full of progress, and the new app must open it, add the column and keep every row.

**A number in the file.** SQLite keeps one spare whole number inside every database file, called `user_version`, and it starts at 0. Apps use it as the **schema version**: the number of the table shape this file has. `version(conn)` in the starter reads it.

**One step per version.** A **migration** is the list of changes from one shape to the next, each guarded by the number: `if v < 1 { create the table }`, `if v < 2 { add the column }`. A brand-new file (version 0) runs every step; a version-1 file runs only the missing one; a file already at 2 runs none.

**Change a table.** `ALTER TABLE quests ADD COLUMN done INTEGER NOT NULL DEFAULT 0` adds a column to a table that already has rows. The `DEFAULT 0` matters: old rows need some value for the new column, and "not done" is the honest one.

```rust
let tx = conn.transaction()?;
let v = version(&tx)?;
if v < 3 {
    tx.execute("ALTER TABLE streaks ADD COLUMN goal INTEGER NOT NULL DEFAULT 1", [])?;
}
tx.pragma_update(None, "user_version", 3)?;
tx.commit()
```

**Write the number back.** After the steps, `tx.pragma_update(None, "user_version", 2)?` records the new version, so the next start skips everything. A **pragma** is SQLite's word for a setting rather than data. `commit()` returns a `Result` itself, so it can be the function's last line.

**All inside one transaction.** If step 2 fails halfway, the rollback from last quest puts the file back exactly as it was: still a valid version-1 file that the old app opens. A half-upgraded file would open in neither.

## Your turn

- In `migrate`, start a transaction and read the version.
- Below 1: create the quests table. Below 2: add the `done` column with a default of 0.
- Write version 2 and commit.
