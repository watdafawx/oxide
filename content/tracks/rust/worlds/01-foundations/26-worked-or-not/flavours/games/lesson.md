A stack cannot hold more than 64 items. When a recipe asks for 100, the calculator has to say no, and "no" is not enough: the player needs to know why. `Option` can say "nothing", but it cannot say why.

**A failure with a reason.** A **Result** is one of two things. `Ok(value)` means it worked and here is the value. `Err(reason)` means it did not, and here is why. This is the `Result` from the reading quest.

**Two types in the arrow.** Its type names both sides: `Result<u32, String>` is "a `u32` if it worked, a `String` reason if it did not". A function that can fail says so in its signature, so a caller can never forget.

**Build the answers.** Wrap the good value in `Ok(..)`. For the failure, make a sentence with `format!` and wrap it in `Err(..)`. Both are plain values, so an `if` can choose between them:

```rust
fn check_fuel(ticks: u32) -> Result<u32, String> {
    if ticks > 1000 {
        Err(format!("{ticks} ticks is too long"))
    } else {
        Ok(ticks)
    }
}
```

**Not a crash.** An `Err` is a normal answer. Nothing stops, nothing panics. The caller looks at the result and decides what to do. The next quest shows how.

## Your turn

- In `validate_stack`, return `Err` with the message when `count` is above 64.
- Otherwise return `Ok(count)`.
- Save. Both objectives turn green.
