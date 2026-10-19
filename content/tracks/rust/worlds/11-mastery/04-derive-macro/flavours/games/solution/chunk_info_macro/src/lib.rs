use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

/// `#[derive(ChunkInfo)]` adds `fn describe(&self) -> String`.
#[proc_macro_derive(ChunkInfo)]
pub fn derive_chunk_info(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let text = match &input.data {
        Data::Struct(s) => {
            let n = s.fields.len();
            format!("{name} with {n} fields")
        }
        Data::Enum(_) => format!("{name} (enum)"),
        Data::Union(_) => format!("{name} (union)"),
    };

    quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            pub fn describe(&self) -> String {
                String::from(#text)
            }
        }
    }
    .into()
}
