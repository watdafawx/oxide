The widget shows a heading only if it has stats under it: a desktop PC without a battery shows no Battery heading at all. `grouped` turns the bar's stats into the list the widget draws: each heading, in the fixed `GROUPS` order, with its stats.

**One pair per heading.** The answer is a `Vec<(Group, Vec<StatKind>)>`: a list of pairs, each a heading and that heading's stats. A tuple in a list is a handy way to keep two things that belong together side by side, without inventing a struct for them.

**Walk the headings.** Go over `GROUPS`, not over the stats: then the headings come out in the fixed order, top to bottom, whatever order the bar has its stats in. For each heading, its stats are the ones `group_of` puts there, taken from `stats` with `.iter().copied().filter(..)` in the order they came.

**Skip the empty ones.** A heading with no stats would draw a lonely title. `continue`, from the Sensors world, skips the rest of the loop body for this heading and moves on to the next one, so nothing is pushed for it.

**The same loop, counting.** Here the same walk counts the stats under each heading, keeping the zeros; yours keeps the stats themselves and leaves the empty headings out:

```rust
let mut counts = Vec::new();
for group in GROUPS {
    let n = stats.iter().filter(|kind| group_of(**kind) == group).count();
    counts.push((group, n));
}
```

**Two stars there.** `filter` lends each item it is given, and `iter()` already gave references, so `kind` there is a `&&StatKind` and needs `**` to reach the value. After `.copied()` the items are plain `StatKind`s again, and the closure gets a `&StatKind` that needs only one `*`.

## Your turn

- In `grouped`, start an empty list and walk `GROUPS`.
- For each heading, collect its stats from `stats`. Skip the heading when there are none; otherwise push the pair.
- Give back the list. Save.
