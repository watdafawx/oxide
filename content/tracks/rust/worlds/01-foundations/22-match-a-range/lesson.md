Orders get a tier from how many extras are added: none is Basic, one or two is Standard, three to five is Premium, and anything more is Deluxe. The numbers are not a few exact values, they are ranges. `match` can match those too.

**Match numbers.** A pattern can be a plain number. `0 => ...` runs when the value is exactly 0, the same way a variant did in the enum quest.

**Match a range.** A pattern can also be a range, written like the ones in a `for` loop: `5..=17` matches 5, 6, up to 17, both ends included.

**Catch the rest.** A match must cover every number, and nobody wants to list billions of them. `_` matches anything that is left. It always goes last.

**Top to bottom.** Rust tries the arms in order and takes the first one that fits. That is why the catch-all must be last: it would swallow everything below it.

```rust
fn ticket_price(age: u32) -> u32 {
    match age {
        0..=4 => 0,
        5..=17 => 5,
        _ => 10,
    }
}
```

## Your turn

- In `tier`, match on `extras`: 0 is `Basic`, 1 to 2 is `Standard`, 3 to 5 is `Premium`, anything else is `Deluxe`.
- Save. Both objectives turn green.
