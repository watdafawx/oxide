Kinetik does not time every window switch. It looks every 5 seconds and gives the program in front 5 seconds of credit. Over a day the error is a few seconds, and the code stays small.

**A day's numbers.** `Day` holds seconds per program in a `BTreeMap`, and seconds per hour of the day in a `[u64; 24]`. The hours feed a chart later.

**Count from zero.** `entry(..).or_insert(0)` gives a mutable count, starting a new program at 0 the first time it is seen:

```rust
*self.apps.entry(app.to_string()).or_insert(0) += SAMPLE_SECS;
```

**Which hour.** `at.hour()` comes from chrono's `Timelike` trait: 0 to 23, ready to index the array.

**The loop is ready.** `ScreenTime::start` spawns a loop that waits 5 seconds, calls `sample`, and notifies anyone watching, like the bar's own tick.

## Your turn

- In `Day::add`, add `SAMPLE_SECS` to the program's seconds.
- Add it to the seconds of `at`'s hour too. Save.
- Press Run: every 5 seconds the terminal prints today's totals.
