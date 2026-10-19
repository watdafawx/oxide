Things go wrong: bad input, missing files, divide by zero. Rust has no exceptions; a function that can fail says so in its return type: **`Result<T, E>`**.

```rust
fn parse_age(text: &str) -> Result<u32, String> {
    match text.trim().parse::<u32>() {
        Ok(age) => Ok(age),
        Err(_) => Err(format!("'{text}' is not an age")),
    }
}

match parse_age("42") {
    Ok(age) => println!("age {age}"),
    Err(msg) => println!("oops: {msg}"),
}

let age = parse_age("42").expect("hard-coded age is valid");
```

- `Ok(value)` = it worked. `Err(reason)` = it didn't, and here is why.
- **`match`** on a Result handles both paths. The compiler won't let you skip `Err`.
- **`expect("why this can't fail")`** gives the value or panics with your message. Use it when failure means a bug.
- The `?` shortcut comes later. For now: `match`.

## Your turn

- Open `src/lib.rs`.
- `safe_divide`: return `Err` when the divisor is 0.
- `describe_division`: match on `safe_divide`. `port_from_config`: use `expect`.
