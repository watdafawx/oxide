**Why now:** every console line starts with a command word (`kick`, `tp`, `give`) followed by arguments, and the `recent` view only shows the tail of the log. Copying text out would waste memory. A **slice** is a borrowed window into part of a string or a list. No copying, no new allocation.

- `&str` is a string slice, `&[T]` is a slice of a list (array or Vec).
- Ranges pick the part: `&x[2..5]` (indexes 2,3,4), `&x[..3]` (first three), `&x[2..]` (from 2 on).
- Slices point into the original, so the original can't change or disappear while you hold one.

```rust
fn first_two(ids: &[u32]) -> &[u32] {
    &ids[..2]
}

let ids = vec![10, 20, 30];
let head = first_two(&ids);       // &Vec<u32> becomes &[u32]
assert_eq!(head, &[10, 20]);

let line = String::from("tp Steve 0 64 0");
let cmd = &line[..2];             // "tp"
```

- `line.find(' ')` returns `Some(index)` of the first space, or `None` when there is none. Slice up to that index.

Out-of-range slices panic, so check the length first.

The `<'a>` in `trim_edges`'s signature is a lifetime label. Ignore it for now: it is given, you only write the body. Quest 09 explains it.

## Your turn

- In `src/lib.rs`, fill in `command_name`, `trim_edges` and `recent_lines`.
- Return slices of the input, not new Vecs or Strings.
