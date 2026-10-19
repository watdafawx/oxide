A **slice** is a borrowed window into part of a string or a list. No copying, no new allocation.

- `&str` is a string slice, `&[T]` is a slice of a list (array or Vec).
- Ranges pick the part: `&x[2..5]` (indexes 2,3,4), `&x[..3]` (first three), `&x[2..]` (from 2 on).
- Slices point into the original, so the original can't change or disappear while you hold one.

```rust
fn first_two(nums: &[i32]) -> &[i32] {
    &nums[..2]
}

let v = vec![10, 20, 30];
let head = first_two(&v);       // &Vec<i32> becomes &[i32]
assert_eq!(head, &[10, 20]);

let s = String::from("rust is fun");
let word = &s[..4];             // "rust"
```

Out-of-range slices panic, so check the length first.

## Your turn

- In `src/lib.rs` fill in `first_word`, `middle` and `last_three`.
- Return slices of the input, not new Vecs or Strings.
