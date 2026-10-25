A line of the stock file has a quantity and a price, as text: `"3"` and `"150"`. The tool needs their product, 450. Either field can be a typo, and then the caller should get the error back instead of a crash. In the Foundations world you did this with a `match` on `Ok` and `Err`; with two fields, that is two matches for one multiplication.

**The long way.** For each field: `match qty.parse::<u32>() { Ok(n) => n, Err(e) => return Err(e) }`. Every fallible step needs the same five lines, and the one line that matters, the multiplication, gets lost between them.

**One character.** **`?`** after a `Result` does exactly that match. If it is `Ok(n)`, the whole expression becomes `n` and the function carries on. If it is `Err(e)`, the function stops and returns `Err(e)` to its caller right there. `?` is an early `return` for errors.

```rust
use std::num::ParseIntError;

fn full_boxes(items: &str, per_box: &str) -> Result<u32, ParseIntError> {
    let items = items.parse::<u32>()?;       // a bad number returns Err here
    let per_box = per_box.parse::<u32>()?;
    Ok(items / per_box)
}
```

**The error type has to fit.** `?` can only return an error the function is allowed to return. `parse::<u32>()` fails with a **`ParseIntError`**, which lives in the standard library at `std::num::ParseIntError`; the `use` line brings it in, like `HashMap`. So the function says `Result<u32, ParseIntError>`.

**Options too.** `?` works on an `Option` in a function that returns an `Option`: `Some(v)` gives `v`, `None` returns `None` at once. `counts.get(0)?` is "the first count, or give up with None".

**For the curious.** The Errors world builds on this: your own error types, and `?` turning one error type into another.

## Your turn

- In `line_value`, parse both fields with `?` and return `Ok` of their product.
- In `first_two_total`, `get` the first two counts with `?` and return `Some` of their sum.
