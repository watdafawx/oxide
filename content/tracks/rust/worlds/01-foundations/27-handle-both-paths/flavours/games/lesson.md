Counts arrive as typed text: `"12"` from a chat command, `"pizza"` from someone not paying attention. Before the calculator can count anything, it has to turn the text into a number, and sometimes that cannot work.

**Text is not a number.** `"12"` is text. The method `.parse::<u32>()` tries to read it as a `u32`, the way you would read it aloud as a number.

**Reading can fail.** If the text is a number you get `Ok(12)`. If it is not, you get an `Err`. Reading something is the classic job that can fail, so Rust never hands the answer back bare.

**Match both paths.** A `match` on the result has one arm for `Ok` and one for `Err`. Rust will not let you skip the `Err` arm, so a typo can never become a crash. In the `Err` arm, `_` means "I do not need the reason":

```rust
fn double_text(text: &str) -> String {
    match text.trim().parse::<u32>() {
        Ok(n) => format!("{}", n * 2),
        Err(_) => format!("'{text}' is not a number"),
    }
}
```

**Trim first.** People type spaces. `.trim()` before `.parse` removes them, so `" 12 "` still reads as 12.

## Your turn

- In `describe_count`, match on `text.trim().parse::<u32>()`.
- For `Ok(n)`, return `12 items` style text. For `Err(_)`, return `'pizza' is not a count`, with the original text in the quotes.
- Save. Both objectives turn green.
