**Why now:** every chunk-ish type in the indexer (`Chunk`, `Section`, `Biome`) wants the same `describe()` method that says what it is. Writing that impl by hand for each is exactly the boilerplate a **derive macro** removes: you write `#[derive(ChunkInfo)]` and the compiler runs your macro to add the impl.

The macro is a function from tokens to tokens, in its own crate (see `chunk_info_macro/`):

```rust
#[proc_macro_derive(ChunkInfo)]
pub fn derive_chunk_info(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);   // syn: tokens -> syntax tree
    let name = &input.ident;                                 // the type's name
    let (impl_g, ty_g, where_c) = input.generics.split_for_impl();

    let text = format!("{name} with a few fields");
    quote! {                                                 // quote: syntax -> tokens
        impl #impl_g #name #ty_g #where_c {
            pub fn describe(&self) -> String { String::from(#text) }
        }
    }
    .into()
}
```

`data` is an enum, so you `match` on it. All three kinds must be covered, and a struct arm can bind its parts (`s.fields` is the list of fields, `.len()` counts them):

```rust
let kind = match &input.data {
    syn::Data::Struct(s) => format!("struct, {} fields", s.fields.len()),
    syn::Data::Enum(_) => String::from("enum"),
    syn::Data::Union(_) => String::from("union"),
};
```
Ident names print with `{}`: `format!("{name}")` gives `Chunk`.

- `DeriveInput` has `ident` (the name), `generics` and `data` (a `Struct`, `Enum` or `Union`).
- `quote!` splices Rust values in with `#name`. Everything else in it is output code.
- `split_for_impl()` returns the three generic fragments you need so `struct Pair<T: Clone>` still works.
- Work out the sentence **outside** `quote!` (it is plain Rust), then splice it in.

## Your turn

In `chunk_info_macro/src/lib.rs`, replace the `TODO` text:

- struct: `"Chunk with 2 fields"` (use `s.fields.len()`)
- enum: `"Biome (enum)"`
- union: anything sensible, the tests do not check it
