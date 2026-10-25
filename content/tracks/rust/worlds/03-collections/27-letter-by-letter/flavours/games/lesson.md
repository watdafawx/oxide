Name tags above players are sized by the number of letters, and item ids like `minecraft:oak_log` may use only small letters, digits, `_` and `:`. Both mean looking at a text one letter at a time, and that is where Rust shows you something most languages hide.

**Letters are not bytes.** Text is stored as **UTF-8**: plain letters like `a` take one byte, but `ë` takes two, and an emoji four. `.len()` counts bytes, so `"Zoë".len()` is 4, though a player sees 3 letters. For a name tag, you want the letters.

**Walk the letters.** `.chars()` is an iterator over the letters, each one a **`char`**: a single character, written in single quotes, `'ë'`. `"Zoë".chars().count()` is 3. Like any iterator, it works with `filter`, `any`, `all` and the rest.

```rust
"café".chars().count()                        // 4 (but "café".len() is 5)
"oak_log".chars().all(|c| c != ' ')           // true: no spaces
'7'.is_ascii_digit()                          // true
```

**Questions a char answers.** A `char` has helpers: `c.is_ascii_lowercase()` for a to z, `c.is_ascii_digit()` for 0 to 9. `chars()` hands out each `char` itself, not a reference, so `all`'s closure needs no `*`.

**For the curious.** Slicing text by position, `&name[0..1]`, counts bytes too. Cut in the middle of `ë` and the server panics, so for letters use `chars()`.

## Your turn

- In `letter_count`, count the `char`s of the name.
- In `is_valid_id`, check the id is not empty and that every char is a small letter, a digit, `'_'` or `':'`.
