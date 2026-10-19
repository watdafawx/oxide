An attribute macro gets the item it sits on and returns whatever should replace it. That makes it the tool for wrapping: logging, timing, retries, call counts.

`#[proc_macro_attribute]` functions take two token streams: the attribute's own arguments, and the item. Parse the item with `syn::ItemFn` (needs syn's `full` feature) and rebuild it around the original body.

```rust
#[proc_macro_attribute]
pub fn timed(_args: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let (vis, sig, block) = (&func.vis, &func.sig, &func.block);
    quote! {
        #vis #sig {
            println!("calling");
            #block
        }
    }
    .into()
}
```

Your `#[timed]` also generates a global counter per function. `#[timed] fn ping()` becomes:

- `static PING_CALLS: AtomicUsize` (uppercase name + `_CALLS`),
- `pub fn ping_calls() -> usize` that reads it,
- `fn ping()` with a `fetch_add(1)` before the original body.

`format_ident!` builds those new names for you.

## Your turn

In `timed_macro/src/lib.rs`, function `timed`:

- Emit the `static` counter.
- Make `{name}_calls()` read it instead of returning 0.
- Bump it at the start of the wrapped function.
