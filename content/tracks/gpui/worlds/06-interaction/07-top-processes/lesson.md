The bottom of Kinetik's hover card lists the processes using the most: which game or tab is eating your memory right now. `real_top_processes` asks sysinfo for every process and its memory; you pick the top five.

**Sort by a key.** `sort_by_key` takes a closure that picks what to sort by: here `p.1`, the memory. That sorts smallest first. Wrapping it in `std::cmp::Reverse` flips the order, biggest first:

```rust
processes.sort_by_key(|p| Reverse(p.1));
```

**Keep five.** `truncate(5)` drops everything after the fifth. A list shorter than five stays as it is.

**Asked only when shown.** Listing every process costs a few milliseconds. The card asks only when it is about to appear, never on a timer; the next quest wires that in.

**Memory, not CPU.** CPU use per process needs two samples a moment apart. Memory is right on the first ask, so the card can show it at once.

## Your turn

- In `top_five`, sort `processes` biggest memory first.
- Keep the first five and give them back.
- Save.
