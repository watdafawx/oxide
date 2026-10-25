A log shows only its newest three entries; a report skips the header and the footer. Copying those items into a new `Vec` would cost a heap allocation each time. As with text, you can point at part of the list instead.

**A slice of a list.** `&list[1..3]` borrows items 1 and 2 of a `Vec` or an array. Its type is `&[i32]` for a list of `i32`s: a pointer to the first item it covers, plus a count.

**Take any list.** A parameter typed `&[i32]` accepts a borrowed `Vec`, an array, or a slice of either. Rust turns a `&Vec<i32>` into a `&[i32]` for you, the same way it turns a `&String` into a `&str`.

```rust
fn first_two(nums: &[i32]) -> &[i32] {
    &nums[..2]
}

let v = vec![10, 20, 30];
let head = first_two(&v);      // [10, 20], still inside v
```

**Check before you cut.** A range past the end panics, just like `a[i]` did in the last world, so check `.len()` first. When the answer is "nothing", give back an **empty slice**, written `&[]`.

**Count back from the end.** The last three start at `len - 3`, which would go below zero for a shorter list. `len().saturating_sub(3)` stops at 0 instead, as it did in "Stop at zero".

## Your turn

- In `middle`, drop the first and the last item; give back `&[]` when there are 2 or fewer.
- In `last_three`, give back the last 3 items, or all of them when there are fewer.
