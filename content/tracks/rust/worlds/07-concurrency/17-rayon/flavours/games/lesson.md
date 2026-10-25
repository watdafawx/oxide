Spawning, splitting, joining and collecting by hand is a lot of code for "do this to every jar". Most of the time you want the iterator chain you already know, run on every core. A crate does exactly that.

**A pool of threads.** **rayon** is a crate, like the ones from the errors world; this quest's `Cargo.toml` already lists it, and the first build downloads it. It keeps a **thread pool**: a fixed set of threads, one per core, started once and reused for every job, so you never pay to start a thread per item.

**Swap one word.** Add `use rayon::prelude::*;`, then write `par_iter()` where you wrote `iter()`. The rest of the chain stays: `map`, `filter`, `sum`, `count`, `collect` all work. Rayon cuts the items into pieces, hands the pieces to the pool, and merges the answers.

**The order is kept.** The pieces finish in any order, but `collect` puts the results back in the order of the input, so `par_iter().map(..).collect()` gives the same `Vec` as the plain chain.

**Lend or give.** As with plain iterators: `par_iter()` lends each item as a `&`, `into_par_iter()` takes the collection and hands out the items themselves, and `par_iter_mut()` lends each item to change it.

**The closure runs everywhere.** Your closure now runs on several threads at once, so it must be `Send` and `Sync`: no `Rc`, no `RefCell`, and no changing a variable from outside the closure. Hand values back instead. And it only pays off when each item is real work: for tiny closures over a short list, the plain `iter()` is faster.

```rust
use rayon::prelude::*;

let jars = vec![String::from("jei.jar"), String::from("sodium.jar")];
let lengths: Vec<usize> = jars
    .par_iter()                  // was: .iter()
    .map(|jar| jar.len())
    .collect();                  // [7, 10], in order
```

## Your turn

- In `checksums`, map every size to `size * 31` in parallel and collect them in order.
- In `total_size`, add up all sizes in parallel.
- In `count_big`, take the `Vec` and count the jars bigger than `limit`, in parallel.
