The planner should show a line like `pen: 12 per box` when it knows the item, and a plain message when it does not. You only care about one of the two cases, the one with a value, and a full `match` is a lot of words for that.

**Two cases, one you want.** With an Option, the interesting case is usually `Some`. The other case just needs a sentence. `match` works, but it is long for a job this small.

**The short way.** `if let Some(n) = per_box(item) { ... }` reads as "if this is a `Some`, call what is inside `n`, and run the block". The name `n` exists only inside the block.

**What if it is None.** Add an `else` block for that case, exactly like an ordinary `if`.

**Still a value.** Like `if` and `match`, the whole `if let` is a value, so both blocks have to give the same type, and it can be the last line of a function.

```rust
if let Some(burn) = fuel_burn("coal") {
    format!("coal burns {burn} ticks")
} else {
    "not a fuel".to_string()
}
```

## Your turn

- In `describe_box`, use `if let Some(n) = per_box(item)`. When it matches, return text like `pen: 12 per box`.
- In the `else` block, return `no box size for unicorn`, with the item's name.
- Save. Both objectives turn green.
