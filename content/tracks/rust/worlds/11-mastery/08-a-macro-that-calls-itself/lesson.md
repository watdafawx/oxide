`std::cmp::max(a, b)` takes exactly two numbers. For a high-score board you want `max!(3, 9, 2)` with as many scores as there are players. A repetition alone cannot compare its items with each other, but a macro may call itself.

**Peel one off.** The biggest of `3, 9, 2` is the bigger of `3` and the biggest of `9, 2`. So an arm can take the first argument as `$x`, the rest as `$( $rest:expr ),+`, and call the same macro on the rest. Each call has one argument fewer, until one is left. Solving a problem through a smaller copy of itself is called **recursion**.

**Base case first.** With one argument the answer is that argument: that arm is the **base case**, where the calls stop. Arms are tried from top to bottom and the first that matches wins. The one-argument arm goes first so a single value never tries the "first plus rest" arm, which needs at least one more.

```rust
#[macro_export]
macro_rules! count {
    () => { 0 };
    ($x:expr $(, $rest:expr)*) => {
        1 + $crate::count!($($rest),*)
    };
}

assert_eq!(count!(7, 7, 7), 3);   // 1 + (1 + (1 + 0))
```

**Expanded, then compiled.** `count!(7, 7, 7)` expands to `1 + count!(7, 7)`, which expands again, four expansions in all, before the compiler sees one long sum. Nothing calls itself while the program runs. The compiler stops a macro that nests deeper than 128 levels.

## Your turn

- In `max!`, write the second arm's template: the larger of `$x` and `max!` of the rest. `::std::cmp::max(a, b)` gives the larger of two.
