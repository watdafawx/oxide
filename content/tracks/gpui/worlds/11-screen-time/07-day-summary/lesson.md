The screen time page opens with a few lines about the day: how long in total, when you started and stopped, which program took the most time, and which hour was busiest. Total, first and last are already in `summarize`; the last two need a search.

**Most of something.** `max_by_key`, from the Rust track's collections world, walks an iterator and keeps the item with the biggest key. On an empty iterator it gives `None`, which is the right answer for an empty day: the page shows `--`.

**One star per reference.** `max_by_key` hands its closure a reference to each item. On an array of pairs, the items are already references, `&(&str, i32)`; the pattern `|(_, count)|` looks through both and leaves `count` as a `&i32`, and one `*` gives the number:

```rust
let launches = [("steam.exe", 3), ("discord.exe", 9)];
launches.iter().max_by_key(|(_, count)| *count) // Some(&("discord.exe", 9))
```

**Pairs from a map.** `day.apps.iter()` gives pairs that hold two references, `(&String, &u64)`, so in `|(_, secs)|` the seconds are `&&u64`, one level deeper. `**secs` reaches the number itself.

**Then the name.** The winner is a pair of references into the day. `.map(|(name, _)| name.clone())` turns it into the `Option<String>` that `most_used` wants, a name of its own.

**Busiest hour.** The hours are the indexes `0..24`, and a range is an iterator of `usize`. `filter` keeps the hours with any seconds; its closure gets a `&usize`, and the pattern `|&hour|` takes the number out of the reference so it can index `day.hours`. Then `max_by_key(|&hour| day.hours[hour])`.

**Why filter first.** Without the filter, an empty day would still pick an hour: all 24 tie at zero, and `max_by_key` gives the last of a tie, 23. An empty day has no busiest hour.

## Your turn

- In `summarize`, fill `most_used` with the name of the program that has the most seconds.
- Fill `busiest_hour` with the hour that has the most seconds, skipping empty hours.
- Save.
