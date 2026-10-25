The printed stock report is a table: names down the left, numbers lined up on the right, prices with exactly two decimals. With plain `{}` every line is a different length and the columns wander. `format!` can pad and round for you.

**Pad to a width.** After a colon inside the braces you can give a width: `{name:<10}` pads `name` with spaces to 10 characters, text on the left. `{qty:>5}` pads to 5, text on the right, which is how numbers line up. Longer values are never cut, only padded.

**Round to decimals.** `{price:.2}` shows an `f64` with exactly two decimals: `1.5` becomes `1.50`, and `3.14159` becomes `3.14`. Width and decimals combine: `{price:>8.2}` is two decimals, right-aligned in 8.

```rust
let staff = "ann";
let hours = 7.5;
format!("{staff:<6}|{hours:>6.1}")    // "ann   |   7.5"
```

**Same as before, plus a colon.** It is still the `format!` you know: the name goes in the braces, and everything after the `:` says how to show it. The same works with `{}` and a value after the text: `format!("{:>5}", qty)`.

**For the curious.** `{name:^10}` centres the text. And the width can come from a variable: `{qty:>width$}` pads to whatever `width` holds, handy when the widest name decides the column.

## Your turn

- In `row`, pad the name to 10 on the left and the qty to 5 on the right.
- In `price_cell`, show the price with 2 decimals, right-aligned in 8.
