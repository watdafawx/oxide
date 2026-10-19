**Why now:** you want to know how often the indexer loads a chunk from disk, without writing counter code in every function. An **attribute macro** receives the whole function you tag and returns a rewritten one: you can add a counter, a log line, a timer.

```rust
#[count_calls]
fn load_chunk(x: i32, z: i32) -> u64 { ... }

// expands to roughly:
static LOAD_CHUNK_CALLS: AtomicUsize = AtomicUsize::new(0);
pub fn load_chunk_calls() -> usize { LOAD_CHUNK_CALLS.load(SeqCst) }
fn load_chunk(x: i32, z: i32) -> u64 {
    LOAD_CHUNK_CALLS.fetch_add(1, SeqCst);
    ... the original body ...
}
```

The macro (in `count_calls_macro/`) parses the function with `syn::ItemFn`, takes it apart (`attrs`, `vis`, `sig`, `block`), and writes it back with `quote!`:

- `#[proc_macro_attribute] fn name(args, item)` gets the attribute's arguments and the item it is on.
- `format_ident!("{}_calls", name)` builds a new identifier from a string.
- `#(#attrs)*` repeats a list of tokens (here: the function's own attributes).
- A `static` with an atomic is a global counter that needs no lock.
- The original `#block` stays untouched, so behaviour does not change.

## Your turn

In `count_calls_macro/src/lib.rs`, fill the three `TODO`s inside `quote!`: declare the static, return its value from `<name>_calls()`, and bump it at the top of the function.
