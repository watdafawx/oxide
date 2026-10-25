Your geometry helpers square numbers everywhere: areas, distances, scores. `x * x` is short, but `(width + border) * (width + border)` is not, and you would rather write `square!(width + border)`. A name with `!` is a macro, like `vec!` and `format!`, and `macro_rules!` lets you write your own.

**Code that writes code.** A **macro** runs before the compiler checks your program: it replaces the call with code. `vec![1, 2]` turns into code that builds a `Vec` and pushes two items, and only then does the compiler look at types.

**Pattern, then template.** `macro_rules!` lists **arms**, each written `(pattern) => { template };`. The compiler matches your call against the pattern and pastes the template in its place. `$x:expr` in a pattern captures one **expression**, any piece of code that has a value (`4`, `1 + 2`, `next()`), and `$x` in the template pastes it back.

```rust
#[macro_export]
macro_rules! double {
    ($x:expr) => {
        $x + $x
    };
}

assert_eq!(double!(21), 42);
assert_eq!(double!(0.25), 0.5);
```

**Why not a function?** `fn square(x: i32) -> i32` takes `i32` only; decimals would need a second function, or a generic one with the `Mul` trait from world 05. A macro never sees a type: it copies code into the template, and the compiler checks the result, so `square!(3)` and `square!(1.5)` both work. That is a small win here; the next quests show things only a macro can do, such as taking any number of arguments.

**One expression stays one.** `$x` pastes a whole expression, not loose text. `double!(1 + 2)` is `(1 + 2) + (1 + 2)`, and with `*` instead of `+` you get 9, not the 5 that `1 + 2 * 1 + 2` would give.

**Share it.** `#[macro_export]` above the macro is an attribute, a note for the compiler (as in world 02's `#[derive]`). It makes the macro usable everywhere in this crate and from other crates. The starter already has it.

## Your turn

- In `square!`, change the template so it multiplies the captured expression by itself.
