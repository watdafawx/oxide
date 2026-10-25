The greeting depends on whether this is someone's first visit, so you build the text inside an `if`. Then you try to hand it back, and Rust says the name does not exist. That is the first ownership rule at work: every value has a place where it ends.

**Try the obvious thing.** Make `greeting` in each branch of the `if`, then give `greeting` back on the last line. Rust refuses to build:

```text
error[E0425]: cannot find value `greeting` in this scope
```

**A block is a room.** Every `{ }` is a **block**: a function body, an `if` branch, a loop body. A name made with `let` inside a block exists from its `let` to that block's closing `}`, and not one line longer. That stretch is the name's **scope**.

**Gone means freed.** When a `String`'s name reaches its `}`, Rust frees the letters on the heap right there. This is called **dropping** the value. The compiler knows the exact `}` for every value before the program runs, which is how Rust frees memory without a garbage collector.

**Let the block hand it back.** You know from the last world that an `if` can be a value: each branch's last line, without `;`, is what the `if` gives back. A value handed out of a block is not dropped. It travels to whoever receives it.

```rust
let label = if full {
    format!("{name}: full")
} else {
    format!("{name}: room left")
};
```

## Your turn

- Save and read the error under `welcome`.
- Make the `if` itself the last line of `welcome`, with a `format!` as the last line of each branch.
