use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

/// `#[derive(ChunkInfo)]` adds `fn describe(&self) -> String`.
#[proc_macro_derive(ChunkInfo)]
pub fn derive_chunk_info(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // TODO: build the sentence from `name` and `input.data`:
    //   struct -> "Chunk with 2 fields"   (use `s.fields.len()`)
    //   enum   -> "Biome (enum)"
    // `match &input.data { syn::Data::Struct(s) => ..., syn::Data::Enum(_) => ..., ... }`
    let text = String::from("TODO");

    quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            pub fn describe(&self) -> String {
                String::from(#text)
            }
        }
    }
    .into()
}
