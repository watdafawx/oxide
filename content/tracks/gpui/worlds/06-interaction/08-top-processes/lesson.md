The bottom of Kinetik's hover card lists the processes using the most: which game or tab is eating your memory right now. `real_top_processes` asks sysinfo for every process and its memory; you pick the top five.

**One process, two parts.** A `ProcessUse` is a pair, `(String, u64)`: the name, then the memory in bytes. For a process `p`, `p.0` is the name and `p.1` the memory.

**Sort by a key.** `sort_by_key` takes a closure that picks what to sort by, and sorts smallest first. Sorting by name, A to Z, would be:

```rust
processes.sort_by_key(|p| p.0.clone());
```

**Biggest first.** `std::cmp::Reverse` wraps a key and flips how it compares, so the biggest key comes first. Sort by `Reverse(..)` of the memory.

**Keep five.** `truncate(5)` drops everything after the fifth. A list shorter than five stays as it is.

**Memory, not CPU.** CPU use per process needs two samples a moment apart. Memory is right on the first ask, so the card can show it at once.

## Your turn

- In `top_five`, sort `processes` biggest memory first.
- Keep the first five and give them back.
- Save.
