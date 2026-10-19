Every game rule you will write is a function: input in, answer out.

```rust
fn double(n: i32) -> i32 {
    let result = n * 2;   // statement: does something, has no value
    result                // expression: no `;`, so it IS the return value
}
```

- Parameters need types: `n: i32`. The return type follows `->`.
- A **statement** ends with `;` and gives nothing back.
- An **expression** has a value. The last expression in the body is returned. Add a `;` and you return `()` instead, which is a compile error here.
- `return x;` exists, but you only need it for early exits.

## Your turn

- Open `src/lib.rs`.
- `square` and `area`: return the product, last line, no semicolon.
- `summary`: call `area` and build the text with `format!`.
