Items get a rarity from how many enchantments they carry: none is Common, one or two is Uncommon, three to five is Rare, and anything more is Epic. The numbers are not a few exact values, they are ranges. `match` can match those too.

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

- In `rarity`, match on `enchants`: 0 is `Common`, 1 to 2 is `Uncommon`, 3 to 5 is `Rare`, anything else is `Epic`.
- Save. Both objectives turn green.
