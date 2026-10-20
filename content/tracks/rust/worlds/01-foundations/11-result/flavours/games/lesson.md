**Why now:** a stack can never hold more than 64, and recipe lines will soon arrive as raw text that may be garbage. Rust has no exceptions; a function that can fail says so in its return type: **`Result<T, E>`**. Your calculator will return `Err(reason)` instead of crashing.

```rust
fn parse_count(text: &str) -> Result<u32, String> {
    match text.trim().parse::<u32>() {
        Ok(count) => Ok(count),
        Err(_) => Err(format!("'{text}' is not a count")),
    }
}

match parse_count("42") {
    Ok(count) => println!("count {count}"),
    Err(msg) => println!("oops: {msg}"),
}

let count = parse_count("42").expect("hard-coded count is valid");
```

- `Ok(value)` = it worked. `Err(reason)` = it did not, and here is why.
- **`match`** on a Result handles both paths. The compiler will not let you skip `Err`.
- `parse()` can work out the number type from context: `let n: u32 = text.parse().expect("a number");` needs no `::<u32>`.
- **`expect("why this cannot fail")`** gives the value or panics with your message. Use it when failure means a bug.
- The `?` shortcut comes later. For now: `match`.

## Your turn

- Open `src/lib.rs`.
- `validate_stack`: return `Err` when the count is above 64.
- `describe_stack`: match on `validate_stack`. `max_stack_from_config`: use `expect`.
