Raw pointers are how Rust talks to C, to hardware, and how `Vec` and `Box` are built. Making one is safe; **using one is not**.

```rust
let mut x = 10;
let p = &mut x as *mut i32;   // safe: casting a reference to a raw pointer

unsafe {
    *p += 1;                  // deref in unsafe
}

let v = [1, 2, 3];
let slice = unsafe { std::slice::from_raw_parts(v.as_ptr(), v.len()) };
assert_eq!(slice, &[1, 2, 3]);
```

- A raw pointer can be null, dangling or unaligned. Rust does not check. **You** do.
- **`from_raw_parts(ptr, len)`** builds a slice. It is valid only if `ptr` is non-null, aligned, points at `len` live items, and nothing mutates them meanwhile.
- Document the rules of an `unsafe fn` in `# Safety`. Put `// SAFETY:` above each unsafe block explaining why the rules hold.
- Best practice: keep the unsafe part tiny and hide it behind a **safe** function that can't be misused. `slice.as_ptr()` plus `slice.len()` always satisfy `from_raw_parts`, so `safe_sum` can be safe.

## Your turn

In `src/lib.rs` (each unsafe fn already has its `# Safety` docs):

- `increment`: add 1 through a `*mut i32`.
- `read_or_none`: `None` when the pointer is null, else the value.
- `sum_raw`: build a slice with `from_raw_parts` and sum it.
- `safe_sum`: the safe wrapper that calls `sum_raw` correctly.
