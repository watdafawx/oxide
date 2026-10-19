`retain(|w| ...)` and `map(|n| ...)` took closures. Time to know what they are: **small unnamed functions that can use variables from around them**.

```rust
let bonus = 10;
let add_bonus = |score: i32| score + bonus;   // captures `bonus`
println!("{}", add_bonus(5));                 // 15

fn run_twice(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}
println!("{}", run_twice(|n| n * 2, 3));      // 12

fn make_multiplier(k: i32) -> impl Fn(i32) -> i32 {
    move |x| x * k                            // `move` takes k with it
}
let triple = make_multiplier(3);
println!("{}", triple(7));                    // 21
```

- `|args| body` is a closure. Types are usually inferred.
- It **captures** variables it uses, so no extra parameters are needed.
- **`impl Fn(A) -> B`** as a parameter type means "any closure or function that takes A and returns B".
- Returning a closure needs `impl Fn(...)` and `move`, because the captured value must survive the call.

## Your turn

- Open `src/lib.rs`.
- `make_adder`, `apply_twice`, `count_matching`: fill in each body.
- `above_limit`: write a closure that uses `limit` from outside.
