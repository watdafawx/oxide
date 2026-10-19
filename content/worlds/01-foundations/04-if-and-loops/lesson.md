Branches decide what happens, loops make it happen again. Rust gives you both as expressions.

```rust
let label = if hp > 50 { "healthy" } else { "hurt" };  // if returns a value

let mut total = 0;
for i in 1..=3 { total += i; }       // 1, 2, 3 (inclusive range)

let mut n = 1;
let first_big = loop {               // loop runs until `break`
    if n * n > 20 { break n; }       // ...and `break` can hand back a value
    n += 1;
};                                   // first_big = 5

while total > 0 { total -= 4; }      // while: repeat as long as true
```

- Conditions are plain `bool`s. No parentheses needed, braces always required.
- `1..4` is 1,2,3. `1..=4` is 1,2,3,4.
- `if` and `else` branches must have the **same type** when you use them as a value.

## Your turn

- Open `src/lib.rs`. Four functions, do them in order.
- `fizzbuzz`: `if / else if` chain returning a `String`.
- `sum_to`: `for` over a range. `collatz_steps`: `while`.
- `first_square_over`: `loop` + `break` with a value.
