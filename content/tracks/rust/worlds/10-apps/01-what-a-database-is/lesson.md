This world builds the engine behind Oxide, the app you are using right now: it remembers every quest, how much XP it gives and whether you finished it, then serves that to a screen. A text file is fine for ten quests. For three hundred, and questions like "which quests are done, biggest XP first?", you want a database.

**A table.** A **database** keeps data in **tables**, like sheets in a spreadsheet. The table `quests` has **columns** (`id`, `title`, `xp`) and one **row** per quest. Every row has the same columns, and each column holds one kind of value: `TEXT` for words, `INTEGER` for whole numbers.

**A key per row.** Two quests can share a title, so each row also gets an `id`. Marking it `INTEGER PRIMARY KEY` asks the database to hand out the numbers itself: 1, 2, 3. The id never repeats, so it points at exactly one row. `NOT NULL` means the column must always have a value.

**Asking in SQL.** You don't open the file and read it; you send the database a sentence in **SQL**, the language databases understand, and it does the work. Five statements cover most apps: `CREATE TABLE` makes a table, `INSERT` adds a row, `SELECT` reads rows, `UPDATE` changes them and `DELETE` removes them.

```sql
CREATE TABLE quests (id INTEGER PRIMARY KEY, title TEXT NOT NULL, xp INTEGER NOT NULL);
INSERT INTO quests (title, xp) VALUES ('Variables', 40);
SELECT title, xp FROM quests WHERE xp > 30 ORDER BY xp DESC;
DELETE FROM quests WHERE id = 3;
```

**Only some rows.** `WHERE` keeps the rows that match a condition, and `ORDER BY` sorts what comes back (`DESC` for biggest first). Text values in SQL sit in single quotes: `'Variables'`. The database answers a `SELECT` with rows, and the other four with "done, N rows changed".

**SQLite.** Most databases are a separate server program. **SQLite** is a whole database in one file, or in memory for tests, with no server to install, which is why apps like Oxide, browsers and phones use it. The `rusqlite` crate lets Rust send it SQL. SQLite has no true/false type, so a flag such as "done" is stored as `0` or `1`.

## Your turn

Answer the three questions. The next quest makes the table for real.
