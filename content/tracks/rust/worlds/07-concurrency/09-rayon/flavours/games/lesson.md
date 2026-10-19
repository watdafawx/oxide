**Why now:** the boss hashes a whole mods folder. You could hand-build a worker pool (and you will for the boss), but for plain "do this to every item" **rayon** needs one word: `iter()` becomes `par_iter()`.

```rust
use rayon::prelude::*;

let sizes = vec![120u64, 340, 560, 80];

let total: u64 = sizes.par_iter().map(|&b| b * 2).sum();     // spread over a thread pool
let doubled: Vec<u64> = sizes.par_iter().map(|&b| b * 2).collect();
println!("{total} {doubled:?}");                              // 2200 [240, 680, 1120, 160]
```

- `par_iter()`, `par_iter_mut()` and `into_par_iter()` mirror the normal iterator methods.
- `collect()` keeps the **order** of the input.
- Only worth it when each item is real work. For tiny closures the thread overhead costs more than it saves.
- The closure must be `Send + Sync`: no `Rc`, no unsynchronised mutation. Rayon enforces what the earlier quests taught.

## Your turn

In `src/lib.rs`:

- `total_bytes`: sum the jar sizes in parallel.
- `name_lengths`: the name length of every jar, in order.
- `count_lucky_ids`: how many ids below `limit` are lucky (prime)? `is_lucky` is given.
