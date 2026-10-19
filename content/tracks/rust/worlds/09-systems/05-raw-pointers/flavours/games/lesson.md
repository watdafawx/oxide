**Why now:** a real pinger gets counts and sizes as pointers into a buffer someone else owns, the way C libraries hand them over. A raw pointer has no lifetime and no guarantee, so dereferencing it is `unsafe`. Your job: keep that `unsafe` small and put a **safe** function around it.

```rust
let mut players = 3;
let p: *mut i32 = &mut players;      // creating a raw pointer is safe

unsafe { *p += 1; }                  // using it is unsafe

let ptr: *const i32 = std::ptr::null();
println!("{}", ptr.is_null());       // true: checking is safe
```

- `*const T` and `*mut T` are raw pointers. They may be null, dangling or unaligned.
- Writing `unsafe fn` means the caller promises something. Document it under `# Safety`.
- `slice::from_raw_parts(ptr, len)` builds a slice view of `len` items. It is unsafe because `ptr` and `len` must describe real memory.
- A safe wrapper takes a `&[T]`: a slice always has a valid pointer and length, so the unsafe call inside can never go wrong.

## Your turn

In `src/lib.rs`:

- `add_player`: add 1 to the `i32` behind a `*mut i32`.
- `player_count_or_none`: `None` for a null pointer, else the value.
- `total_players_raw`: sum `len` items starting at a pointer.
- `total_players`: the safe version over a `&[i32]`.
