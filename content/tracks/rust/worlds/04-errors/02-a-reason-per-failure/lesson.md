Each task in the todo file has a priority from 1 to 5. A typed priority can be wrong in three ways: nothing there, a word like `high`, or a number above 5. With a `String` error, the caller gets a sentence and has to guess which of the three happened.

**A sentence hides the kind.** `Err("'high' is not a number".to_string())` reads well, but code cannot ask it "were you empty?". Comparing the text would break the day someone rewords the message.

**One variant per failure.** An error can be an `enum`, like any other choice between a few things. Each variant names one way to fail: `Empty`, `NotANumber`, `TooHigh`. The function returns `Result<u32, PriorityError>`: a priority, or one of those.

**Carry the details.** A variant can hold data, as in the foundations world. `NotANumber(String)` keeps the text that was typed and `TooHigh(u32)` keeps the number, so the message can show them later.

**Leave early.** `return Err(...)` ends the function right there, inside an `if` or inside a `match` arm. The checks then read top to bottom, and the last line is the `Ok` that only good input reaches.

```rust
enum DueError {
    NoDate,
    PastMonthEnd(u32),
}

fn check_day(day: u32) -> Result<u32, DueError> {
    if day == 0 {
        return Err(DueError::NoDate);
    }
    if day > 31 {
        return Err(DueError::PastMonthEnd(day));
    }
    Ok(day)
}
```

**The caller picks.** Now a caller can `match` the error: skip the line on `Empty`, but warn the user on `TooHigh(n)`. That choice is why the error is an enum and not a sentence.

## Your turn

- In `parse_priority`, trim the text, then return `Err(PriorityError::Empty)` when nothing is left.
- Parse it with `parse::<u32>()`; on `Err(_)` return `NotANumber` carrying the trimmed text.
- A number above 5 is `TooHigh(n)`. Anything else is `Ok(n)`.
