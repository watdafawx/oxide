A count of crates is never below zero. The change in a count can be: drop five crates and the change is -5. Rust has a different type of whole number for each job, and it makes you pick one.

**Counts and the u.** Every number has a **type**, which says what sort of number it is. You have been writing `u32`. The `u` stands for **unsigned**, which means no minus sign, so the number is 0 or more. That is the right type for a count.

**Changes need a sign.** `i32` is **signed**: it can hold -5 as well as 5. Use it for a difference or a change, anything that can go below zero.

**Try the obvious thing.** Write a function that takes two `i32` numbers, returns `after - before`, and give it the return type `u32`. Rust refuses before the program ever runs:

```text
error[E0308]: mismatched types
  |     after - before
  |     ^^^^^^^^^^^^^^ expected `u32`, found `i32`
```

**Types, not values.** Rust did not work out that the answer could be negative; it never looks at the numbers. It only compared the type you promised (`u32`) with the type you produced (an `i32` minus an `i32` is an `i32`). The return type is a promise about every value the function could ever give back. A change can be negative, so its promise is `i32`.

**Small types.** The number in the name is how much room the type has. `u8` holds 0 to 255. `i8` holds -128 to 127. Put 200 in an `i8` and the compiler says it does not fit. Pick the smallest type that holds every value you need.

```rust
let count: u32 = 64;    // 0 and up
let change: i32 = -5;   // can go below zero
let code: u8 = 200;     // 0 to 255
```

**Writing the type.** A colon and the type after a name, `let count: u32`, says which type you want. Most of the time Rust works it out, so you only write it when it matters.

## Your turn

- Save and read the first error. It points at `stock_change`.
- Fix the return type of `stock_change` so a drop can be negative. Save again.
- A second error now points at `shelf_code`. The compiler says what to use instead.
