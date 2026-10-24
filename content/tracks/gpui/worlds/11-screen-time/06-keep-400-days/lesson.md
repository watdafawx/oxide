A year of day files is about 365 small files. Kinetik keeps 400 days, enough to compare this month with the same month last year, and deletes older ones when a new day starts. The only safe way to know a file's day is its name.

**One entry at a time.** `std::fs::read_dir(dir)?` lists the folder. Each `entry` is itself an `io::Result`, because a file can vanish while the folder is being read, so `entry?.path()` gives its path.

**The name without .json.** `path.file_stem()` gives the file name without its extension, as an `Option<&OsStr>`. An `OsStr` is the operating system's own kind of text, which on Windows does not have to be valid UTF-8, so `.to_str()` turns it into a `&str`, or `None`. Both steps may come up empty, so they chain with `and_then`, as in the dashboard's card summary.

**Back to a date.** `NaiveDate::parse_from_str(stem, "%Y-%m-%d")` reads a date in the pattern `day_path` wrote, and `.ok()` turns its `Result` into an `Option`. The name, never the file's modified time: copying or restoring a file changes that time, but not its name.

```rust
NaiveDate::parse_from_str("2025-09-17", "%Y-%m-%d").ok() // Some(2025-09-17)
NaiveDate::parse_from_str("notes", "%Y-%m-%d").ok()      // None
```

**Only day files.** The folder is yours too: a `notes.txt` or an oddly named file is left alone. `path.extension().is_some_and(|ext| ext == "json")` checks the extension, with the `is_some_and` from the widget world.

**Days between.** Subtracting two dates gives a `TimeDelta`, and `num_days()` is the whole days in it: `(today - date).num_days() > KEEP_DAYS` is true for a file 401 days old.

**One if, three tests.** The `&&` chain from the Settings world takes a `let` too: the extension check `&& let Some(date) = date &&` the age. The body runs only for a parsed day file that is old enough: `std::fs::remove_file(&path)?`, then `deleted += 1`.

## Your turn

- In `prune`'s loop, take each entry's path and turn its stem into an `Option<NaiveDate>`.
- In one `if`, remove `json` files more than `KEEP_DAYS` before `today`, and count them in `deleted`.
- Save.
