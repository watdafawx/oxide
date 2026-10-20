**Why now:** a calculator has to decide ("is this slot full?") and repeat ("add up every step of the staircase"). Rust gives you both as expressions, so a whole `if` can be the answer.

```rust
let label = if count == 64 { "full" } else { "room left" };  // if returns a value

let mut total = 0;
for i in 1..=3 { total += i; }       // 1, 2, 3 (inclusive range)

let mut radius = 1;
let reach = loop {                   // loop runs until `break`
    if radius >= 10 { break radius * 16; }   // ...and `break` can hand back a value
    radius += 1;
};                                   // reach = 160

while total > 0 { total -= 4; }      // while: repeat as long as true
```

- Conditions are plain `bool`s. No parentheses needed, braces always required.
- `1..4` is 1,2,3. `1..=4` is 1,2,3,4.
- `if` and `else` branches must have the **same type** when you use them as a value.
- A parameter can be `mut` too: `fn f(mut n: i32)` gives the function its own changeable copy of `n`. The caller's value is untouched.

## Your turn

- Open `src/lib.rs`. Three functions, do them in order.
- `slot_label`: `if / else if` chain returning a `String`.
- `staircase_blocks`: `for` over a range. `swings_to_break`: `while`.
