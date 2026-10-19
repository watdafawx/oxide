**Why now:** every calculator rule is a function: items in, answer out. "How many items are in 3 stacks of 64?" is `item_count(3, 64)`, and bigger answers are built by calling smaller ones.

```rust
fn double_stack(n: u32) -> u32 {
    let result = n * 2;   // statement: does something, has no value
    result                // expression: no `;`, so it IS the return value
}
```

- Parameters need types: `n: u32`. The return type follows `->`.
- A **statement** ends with `;` and gives nothing back.
- An **expression** has a value. The last expression in the body is returned. Add a `;` and you return `()` instead, which is a compile error here.
- `return x;` exists, but you only need it for early exits.

## Your turn

- Open `src/lib.rs`.
- `grid_slots` and `item_count`: return the product, last line, no semicolon.
- `inventory_line`: call `item_count` and build the text with `format!`.
