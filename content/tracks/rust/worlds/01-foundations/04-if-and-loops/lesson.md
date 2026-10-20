Branches decide what happens, loops make it happen again. Rust gives you both as expressions.

```rust
let label = if hp > 50 { "healthy" } else { "hurt" };  // if returns a value

let mut total = 0;
for i in 1..=3 { total += i; }       // 1, 2, 3 (inclusive range)

let mut size = 1;
let first_big = loop {               // loop runs until `break`
    if size >= 100 { break size; }   // ...and `break` can hand back a value
    size *= 2;
};                                   // first_big = 128

while total > 0 { total -= 4; }      // while: repeat as long as true
```

- Conditions are plain `bool`s. No parentheses needed, braces always required.
- `1..4` is 1,2,3. `1..=4` is 1,2,3,4.
- `.to_string()` turns a number or a `&str` into an owned `String`: `7.to_string()` is `"7"`, `"Full".to_string()` is a `String`. A bare `"Full"` is only a `&str`, so a function that returns `String` needs it.
- `%` is the remainder (`7 % 3` is 1), `!=` means "not equal", and `n /= 2` halves `n` in place, just like `+=` adds in place.
- `if` and `else` branches must have the **same type** when you use them as a value.
- A parameter can be `mut` too: `fn f(mut n: i32)` gives the function its own changeable copy of `n`. The caller's value is untouched.

## Your turn

- Open `src/lib.rs`. Three functions, do them in order.
- `fizzbuzz`: `if / else if` chain returning a `String`.
- `sum_to`: `for` over a range. `collatz_steps`: `while`.
