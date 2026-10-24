The `/inv` command prints a table in chat: item names down the left, counts lined up on the right, jar sizes with one decimal. With plain `{}` every line is a different length and the columns wander. `format!` can pad and round for you.

**Pad to a width.** After a colon inside the braces you can give a width: `{name:<12}` pads `name` with spaces to 12 characters, text on the left. `{count:>4}` pads to 4, text on the right, which is how numbers line up. Longer values are never cut, only padded.

**Round to decimals.** `{size_mb:.1}` shows an `f64` with exactly one decimal: `20.0` stays `20.0`, and `1.26` becomes `1.3`. Width and decimals combine: `{size_mb:>7.1}` is one decimal, right-aligned in 7.

```rust
let player = "Alex";
let health = 17.5;
format!("{player:<6}|{health:>6.2}")    // "Alex  | 17.50"
```

**Same as before, plus a colon.** It is still the `format!` you know: the name goes in the braces, and everything after the `:` says how to show it. The same works with `{}` and a value after the text: `format!("{:>4}", count)`.

**For the curious.** `{name:^12}` centres the text. And the width can come from a variable: `{count:>width$}` pads to whatever `width` holds, handy when the longest item name decides the column.

## Your turn

- In `inventory_row`, pad the name to 12 on the left and the count to 4 on the right.
- In `size_cell`, show the size with 1 decimal, right-aligned in 7.
