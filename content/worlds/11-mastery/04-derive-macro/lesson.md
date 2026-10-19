**Why now:** every block type in the chunk indexer needs the same boilerplate `impl`, and a derive macro writes it from the struct.

A derive macro reads a struct or enum and writes an `impl` for it. You build `#[derive(Hello)]`, which adds `fn hello(&self) -> String`.

The recipe has three steps:

1. **Parse**: `parse_macro_input!(input as DeriveInput)` gives you `ident` (the type name), `generics` and `data` (struct, enum or union).
2. **Compute**: look at `data`. For a struct, `s.fields.len()` is the field count.
3. **Generate**: `quote! { impl ... }` builds tokens. `#name` and `#text` splice Rust values in.

Generics need care: `split_for_impl()` hands you the three pieces of `impl<T> Name<T> where ...`.

```rust
let (impl_g, ty_g, where_c) = input.generics.split_for_impl();
quote! {
    impl #impl_g #name #ty_g #where_c {
        pub fn hello(&self) -> String { String::from(#text) }
    }
}
```

This quest is a cargo workspace: the root crate uses the macro, `derive_hello/` defines it.

## Your turn

In `derive_hello/src/lib.rs`, function `derive_hello`:

- Replace the placeholder `text` with a real one: `Point with 2 fields` for a struct, `Shape (enum)` for an enum.
- Everything else (generics, `quote!`) is already wired.
