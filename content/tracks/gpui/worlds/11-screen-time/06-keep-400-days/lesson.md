A year of day files is about 365 small files. Kinetik keeps 400 days, enough to compare this month with the same month last year, and deletes older ones when a new day starts.

**Dates in names.** `file_stem` is the name without `.json`; `NaiveDate::parse_from_str` turns `2025-09-17` back into a date, or fails for anything that is not a day file.

**Only day files.** The folder is yours too. A `notes.txt` or a file named oddly is skipped: the extension must be `json` and the name must parse.

**Days between.** Subtracting two dates gives a `TimeDelta`; `num_days()` is the whole days between them:

```rust
(today - date).num_days() > KEEP_DAYS
```

**One `if`, three tests.** `&&` chains plain conditions with `let Some(date) = date`, so the body only runs for a parsed day file that is old enough.

## Your turn

- In `prune`'s loop, take each entry's path and parse its stem as a date.
- Remove `json` files more than `KEEP_DAYS` before `today`, counting them in `deleted`.
- Save.
