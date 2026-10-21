A fragile pallet holds only 10 boxes, while a normal one holds 64. The planner has to choose between two numbers depending on a fact about the pallet. That choice is an `if`.

**A yes or no.** A comparison like `weight <= 5` gives back `true` or `false`. A value that is one or the other is a **bool**. The comparisons are `==` (equal), `!=` (not equal), `<`, `>`, `<=` and `>=`. A single `=` gives a name a value; two `==` ask a question.

**Pick one.** `if` takes a bool and runs the first block when it is true, or the `else` block when it is not. A function that gets `is_fragile: bool` can answer with `if is_fragile { 10 } else { 64 }`.

**An if is a value.** The block that runs is the answer, with no semicolon, exactly like a function's last line. That is why the whole `if` can sit on the last line of a function. Both blocks have to give the same type of value.

**More than two choices.** Add `else if` between them. Rust tries the conditions from the top and takes the first one that is true; `else` catches everything left.

```rust
fn parcel_size(weight: u32) -> u32 {
    if weight == 1 {
        2
    } else if weight == 2 {
        4
    } else {
        6
    }
}
```

## Your turn

- In `pallet_limit`, return 10 for a fragile pallet and 64 otherwise.
- In `shipping_fee`, return 5 up to 1 kg, 8 up to 5 kg, and 12 for anything heavier.
- Save. Both objectives turn green.
