**Why now:** your loot table is about to get long, and the next quests turn it into answers: which drops are rare, what is the total weight. Iterator chains do that in one readable line, and one rule makes them click.

**Iterators are lazy.** Adaptors like `map` and `filter` don't do any work. They just describe what *will* happen, like writing a recipe without cooking. Nothing runs until a **consumer** pulls the items out.

```rust
let weights = vec![50, 30, 10, 2];

let plan = weights.iter().map(|w| w * 2);   // nothing computed yet!

let doubled: Vec<i32> = plan.collect();     // NOW it runs: [100, 60, 20, 4]
let total: i32 = weights.iter().sum();      // consumer: 92
let heavy = weights.iter().filter(|w| **w > 20).count();   // consumer: 2
```

- **Adaptors** (lazy, return another iterator): `map`, `filter`, `enumerate`, `zip`, `take`, `skip`, `rev`.
- **Consumers** (run the chain, return a result): `collect`, `sum`, `count`, `fold`, `find`, `any`, `max`, `for` loops.
- The compiler warns `unused ... must be used` if you build a chain and drop it. That warning is this rule talking.
- Why lazy? `drops.iter().map(..).take(3)` only does the work for 3 drops, not the whole table.

## Quiz

Two questions below. The code above has both answers.
