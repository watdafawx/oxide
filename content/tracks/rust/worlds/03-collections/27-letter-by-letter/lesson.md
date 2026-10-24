Shelf labels are printed with a fixed number of letters per line, and product codes like `MUG-04` must use only capitals, digits and dashes. Both mean looking at a text one letter at a time, and that is where Rust shows you something most languages hide.

**Letters are not bytes.** Text is stored as **UTF-8**: plain letters like `a` take one byte, but `é` takes two, and an emoji four. `.len()` counts bytes, so `"café".len()` is 5, though a person sees 4 letters. For a label, you want the letters.

**Walk the letters.** `.chars()` is an iterator over the letters, each one a **`char`**: a single character, written in single quotes, `'é'`. `"café".chars().count()` is 4. Like any iterator, it works with `filter`, `any`, `all` and the rest.

```rust
"Zoë".chars().count()                         // 3 (but "Zoë".len() is 4)
"A1-B".chars().all(|c| c != ' ')              // true: no spaces
'7'.is_ascii_digit()                          // true
```

**Questions a char answers.** A `char` has helpers: `c.is_ascii_uppercase()` for A to Z, `c.is_ascii_digit()` for 0 to 9. `chars()` hands out each `char` itself, not a reference, so `all`'s closure needs no `*`.

**For the curious.** Slicing text by position, `&name[0..1]`, counts bytes too. Cut in the middle of `é` and the program panics, so for letters use `chars()`.

## Your turn

- In `letter_count`, count the `char`s of the name.
- In `is_valid_code`, check the code is not empty and that every char is a capital, a digit or `'-'`.
