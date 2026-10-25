Kinetik does not time every window switch. It looks every 5 seconds and gives the program in front 5 seconds of credit. Over a day the error is a few seconds, and the code stays small.

**A day's numbers.** `Day` holds seconds per program in `apps`, a `BTreeMap<String, u64>`, and seconds per hour of the day in `hours`, a `[u64; 24]`. The hours feed a chart later in this world.

**Count from zero.** `entry(key).or_insert(0)` finds the program's count, or adds it at 0 the first time the program is seen. Either way it gives back a `&mut u64`: a reference to the number inside the map, not a copy.

**Through the reference.** To change the number a reference points at, put `*` in front, the **dereference**: "the thing this points to". `*count += 5` adds 5 to the number in the map; `count += 5` would try to add 5 to the reference itself, which Rust refuses. Counting launches works the same way:

```rust
*launches.entry("steam.exe".to_string()).or_insert(0) += 1;
```

**Which hour.** `at.hour()` comes from chrono's `Timelike` trait and gives 0 to 23. Arrays are indexed with a `usize`, so `at.hour() as usize` picks the slot in `hours`.

**The loop is ready.** `ScreenTime::start` spawns a loop that waits 5 seconds, calls `sample`, and notifies anyone watching, like the bar's own tick. `add` already keeps the first and last second of use.

## Your turn

- At the top of `Day::add`, add `SAMPLE_SECS` to this program's seconds in `apps`.
- Add it to the seconds of `at`'s hour in `hours` too. Save.
- Press Run: every 5 seconds the terminal prints today's totals.
