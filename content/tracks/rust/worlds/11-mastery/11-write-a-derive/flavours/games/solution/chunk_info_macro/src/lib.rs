use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

/// `#[derive(ChunkInfo)]` adds `fn describe(&self) -> String` to a struct or enum.
#[proc_macro_derive(ChunkInfo)]
pub fn derive_chunk_info(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let text = sentence(&input);

    quote! {
        impl #name {
            pub fn describe(&self) -> String {
                String::from(#text)
            }
        }
    }
    .into()
}

/// "Chunk with 2 fields" for a struct, "Biome (enum)" for an enum. Already written.
fn sentence(input: &DeriveInput) -> String {
    let name = &input.ident;
    match &input.data {
        Data::Struct(s) => format!("{name} with {} fields", s.fields.len()),
        Data::Enum(_) => format!("{name} (enum)"),
        Data::Union(_) => format!("{name} (union)"),
    }
}
