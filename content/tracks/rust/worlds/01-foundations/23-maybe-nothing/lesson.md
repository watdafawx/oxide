Ask the box book how many pens fit in a box and you get 12. Ask about a unicorn and there is no answer at all. A lookup that might find nothing has to say so, and Rust does not have a `null` for it.

**Maybe an answer.** Rust's answer is **Option**. A value of type `Option<u32>` is one of two things: `Some(12)`, a number inside a wrapper, or `None`, which means no number. This is the `Option` the reading quest promised.

**Say it in the arrow.** A function that may find nothing returns `Option<u32>` instead of `u32`. The caller can see it right in the signature, so nobody forgets the empty case.

**Match text.** A `match` can compare text with text in quotes, and `_` catches any other text, as it caught any other number. Each arm hands back `Some(..)`, or `None` for the rest.

```rust
fn fuel_burn(item: &str) -> Option<u32> {
    match item {
        "coal" => Some(80),
        "stick" => Some(5),
        _ => None,
    }
}
```

**Not a number yet.** `Some(12)` is a wrapper around 12, not 12 itself. The next quest shows how to get the number out.

## Your turn

- In `per_box`, match on `item`: pens are `Some(12)`, mugs `Some(6)`, lamps `Some(1)`, and anything else is `None`.
- Save. Both objectives turn green.
