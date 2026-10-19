Next up: iterator chains, Rust's way to say "take this list, transform it, filter it, total it" in one readable line. One rule makes them click.

**Iterators are lazy.** Adaptors like `map` and `filter` don't do any work. They just describe what *will* happen. Nothing runs until a **consumer** pulls the items out.

```rust
let numbers = vec![1, 2, 3, 4];

let plan = numbers.iter().map(|n| n * 2);   // nothing computed yet!

let doubled: Vec<i32> = plan.collect();     // NOW it runs: [2, 4, 6, 8]
let total: i32 = numbers.iter().sum();      // consumer: 10
let how_many = numbers.iter().filter(|n| **n > 2).count();   // consumer: 2
```

- **Adaptors** (lazy, return another iterator): `map`, `filter`, `enumerate`, `zip`, `take`, `skip`, `rev`.
- **Consumers** (run the chain, return a result): `collect`, `sum`, `count`, `fold`, `find`, `any`, `max`, `for` loops.
- The compiler warns `unused ... must be used` if you build a chain and drop it. That warning is this rule talking.
- Why lazy? `iter().map(..).take(3)` only does the work for 3 items, not the whole list.

## Quiz

Two questions below. The code above has both answers.
