Your calculator's `add` is called from many places, and you want to know how often, without writing counter code in its body or in every caller. An attribute macro can do it: `#[counted]` above the function, and a `calls()` that tells you the count.

**Gets it, replaces it.** A function marked `#[proc_macro_attribute]` takes two token streams: what is inside the attribute's own parentheses (empty for `#[counted]`), and the item it sits on. What it returns takes the item's place. Return the item unchanged and nothing happens; return nothing and the function is gone.

**Take it apart.** `parse_macro_input!(item as ItemFn)` parses a function (syn's `full` feature, on in this crate's `Cargo.toml`). An `ItemFn` has four parts: `attrs`, the attributes above it such as `///` docs; `vis`, its `pub` or nothing; `sig`, the signature `fn add(a: i32, b: i32) -> i32`; and `block`, the body in its braces.

**Put it back together.** In `quote!`, `#vis #sig { ... }` writes the function again, and you choose what goes in the new body: your line first, then `#block`, the original body, so the function still returns what it did. `#(#attrs)*` is quote's repetition: one copy of each attribute, like `$( )*` in `macro_rules!`.

```rust
#[proc_macro_attribute]
pub fn logged(_args: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let (attrs, vis, sig, block) = (&func.attrs, &func.vis, &func.sig, &func.block);
    quote! {
        #(#attrs)*
        #vis #sig {
            println!("calling");
            #block
        }
    }
    .into()
}
```

**A counter without a lock.** The starter's macro already writes the counter next to the function: `static CALLS`, an `AtomicUsize` from world 07's atomics quest, one number that any thread may change safely. `CALLS.fetch_add(1, Ordering::SeqCst)` adds 1. Inside `quote!` the full `::std::sync::atomic::...` paths are spelled out, for the reason you met in quest 06.

## Your turn

- In `counted_macro/src/lib.rs`, replace `#func` with the function rebuilt from `attrs`, `vis`, `sig` and a new body.
- The new body adds 1 to `CALLS` first, then runs `#block`.
