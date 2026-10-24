`std::cmp::max(a, b)` takes exactly two numbers. The chunk indexer wants the tallest column in a chunk, `tallest!(64, 120, 90)`, with as many heights as there are columns. A repetition alone cannot compare its items with each other, but a macro may call itself.

**Peel one off.** The tallest of `64, 120, 90` is the taller of `64` and the tallest of `120, 90`. So an arm can take the first argument as `$x`, the rest as `$( $rest:expr ),+`, and call the same macro on the rest. Each call has one argument fewer, until one is left. Solving a problem through a smaller copy of itself is called **recursion**.

**Base case first.** With one argument the answer is that argument: that arm is the **base case**, where the calls stop. Arms are tried from top to bottom and the first that matches wins. The one-argument arm goes first so a single value never tries the "first plus rest" arm, which needs at least one more.

```rust
#[macro_export]
macro_rules! count {
    () => { 0 };
    ($x:expr $(, $rest:expr)*) => {
        1 + $crate::count!($($rest),*)
    };
}

assert_eq!(count!("stone", "dirt", "sand"), 3);   // 1 + (1 + (1 + 0))
```

**Expanded, then compiled.** `count!("stone", "dirt", "sand")` expands to `1 + count!("dirt", "sand")`, which expands again, four expansions in all, before the compiler sees one long sum. Nothing calls itself while the program runs. The compiler stops a macro that nests deeper than 128 levels.

## Your turn

- In `tallest!`, write the second arm's template: the larger of `$x` and `tallest!` of the rest. `::std::cmp::max(a, b)` gives the larger of two.
