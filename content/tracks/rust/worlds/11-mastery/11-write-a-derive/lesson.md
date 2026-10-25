Every type in your shapes library wants a `describe()` that says what it is: "Point with 2 fields", "Shape (enum)". Written by hand, that is one more `impl` per type, out of date the day someone adds a field. A derive writes it from the type itself, every time it builds. This starter does not compile yet: the tests call `describe()`, and until your macro writes it, the method does not exist (`error[E0599]: no method named describe`).

**Parse the input.** The derive gets the type's tokens. `parse_macro_input!(input as DeriveInput)` asks syn to parse them as a type definition, or to stop with a clear compile error if they are not one. A `DeriveInput` has `ident`, the type's name, `generics`, and `data`, which says struct, enum or union.

**Read it.** The starter's `sentence` helper already reads `data`: a struct's `fields.len()` is its field count, an enum gets "(enum)". That part is ordinary Rust, run inside the compiler.

**Write the output.** `quote! { ... }` takes Rust code and gives back its tokens. Everything inside is output, written as you would type it, except `#name`: a `#` in front of a variable splices that value in. An `Ident` lands as a name, a `String` lands as a string literal in quotes.

```rust
#[proc_macro_derive(TypeName)]
pub fn derive_type_name(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let text = name.to_string();
    quote! {
        impl #name {
            pub fn type_name() -> &'static str {
                #text
            }
        }
    }
    .into()
}
```

**Two kinds of TokenStream.** The compiler's own `proc_macro::TokenStream` only exists inside a macro crate. syn and quote use a copy, `proc_macro2::TokenStream`, that works in any crate, including their own tests. `.into()` at the end turns quote's stream into the compiler's.

## Your turn

- In `describe_macro/src/lib.rs`, replace the stub in `derive_describe` with a `quote!` block: an `impl` for `#name` whose `describe(&self) -> String` returns `#text` as a `String`.
- Turn the result into a `TokenStream` with `.into()`.
