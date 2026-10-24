`count_long_names` counts the block names in a scan line that are at least some length, to size the legend. It copies every name into a new `String`, collects them into a new `Vec`, and only then counts. The test allows zero allocations, because counting needs none.

**Slices are free.** `split_whitespace` hands out `&str` pieces that point into the line you already have (world 02's string slices): an address and a length each, no copy. `String::from` on each piece copies its letters onto the heap, one allocation per name, to look at them once.

**Only collect allocates.** An iterator chain does its work one item at a time as the last step pulls it (world 03). `filter` passes some items on, `count` adds them up; neither stores anything. `collect` is the step that builds a container, and that container needs heap memory.

```rust
let props = "# comment\nview-distance=16\n# another\nseed=42";
let settings = props
    .lines()
    .filter(|line| !line.starts_with('#'))
    .count();                 // 2, and nothing allocated
```

**The habit.** Take `&str` and give out `&str` while you only read text; make a `String` when you must keep or change it. The same goes for `&[T]` over `Vec<T>` in parameters.

## Your turn

- In `count_long_names`, count straight from `split_whitespace` with `filter` and `count`: no `String`, no `collect`.
