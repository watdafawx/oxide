`/give Alex diamond 3 64` asks for 3 stacks of 64, typed as text: `"3"` and `"64"`. The plugin needs their product, 192. Either argument can be a typo, and then the command should answer with the error instead of crashing the server. In the Foundations world you did this with a `match` on `Ok` and `Err`; with two arguments, that is two matches for one multiplication.

**The long way.** For each argument: `match stacks.parse::<u32>() { Ok(n) => n, Err(e) => return Err(e) }`. Every fallible step needs the same five lines, and the one line that matters, the multiplication, gets lost between them.

**One character.** **`?`** after a `Result` does exactly that match. If it is `Ok(n)`, the whole expression becomes `n` and the function carries on. If it is `Err(e)`, the function stops and returns `Err(e)` to its caller right there. `?` is an early `return` for errors.

```rust
use std::num::ParseIntError;

fn full_chests(items: &str, per_chest: &str) -> Result<u32, ParseIntError> {
    let items = items.parse::<u32>()?;           // a bad number returns Err here
    let per_chest = per_chest.parse::<u32>()?;
    Ok(items / per_chest)
}
```

**The error type has to fit.** `?` can only return an error the function is allowed to return. `parse::<u32>()` fails with a **`ParseIntError`**, which lives in the standard library at `std::num::ParseIntError`; the `use` line brings it in, like `HashMap`. So the function says `Result<u32, ParseIntError>`.

**Options too.** `?` works on an `Option` in a function that returns an `Option`: `Some(v)` gives `v`, `None` returns `None` at once. `hotbar.get(0)?` is "the first stack, or give up with None".

**For the curious.** The Errors world builds on this: your own error types, and `?` turning one error type into another.

## Your turn

- In `items_in`, parse both arguments with `?` and return `Ok` of their product.
- In `first_two_slots`, `get` the first two stacks with `?` and return `Some` of their sum.
