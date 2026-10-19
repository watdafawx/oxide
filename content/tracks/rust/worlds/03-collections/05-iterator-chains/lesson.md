Loops with a `mut` Vec and a counter work, but they bury the point. An **iterator chain** says what you want in a line: *take these, change them, keep some, give me a result*.

```rust
let numbers = vec![1, 2, 3, 4, 5, 6];

let doubled: Vec<i32> = numbers.iter().map(|n| n * 2).collect();
// [2, 4, 6, 8, 10, 12]

let big: Vec<i32> = numbers.iter().copied().filter(|n| *n > 3).collect();
// [4, 5, 6]

let total: i32 = numbers.iter().sum();                   // 21
let evens = numbers.iter().filter(|n| *n % 2 == 0).count();   // 3
```

- **`.iter()`** starts a chain by borrowing each item (`&i32`).
- **`.map(|x| ...)`** changes each item. **`.filter(|x| ...)`** keeps items where the closure is `true`.
- **`.copied()`** turns `&i32` into `i32`, handy before `filter`/`collect`.
- **`.collect()`** builds the result. Say what you want: `let v: Vec<i32> = ...` or `collect::<Vec<_>>()`.
- **`.sum()`** and **`.count()`** end the chain with a single number.

## Your turn

- Open `src/lib.rs`.
- Each of the four functions is one chain. No `for` loops, no `mut`.
