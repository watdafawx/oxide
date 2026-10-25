use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

/// `#[derive(Describe)]` adds `fn describe(&self) -> String` to a struct or enum.
#[proc_macro_derive(Describe)]
pub fn derive_describe(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let text = sentence(&input);
    // TODO: a generic type needs its generics in the impl line. Ask `input.generics` for the
    // three pieces (impl generics, type generics, where clause) and splice each into its place.

    quote! {
        impl #name {
            pub fn describe(&self) -> String {
                String::from(#text)
            }
        }
    }
    .into()
}

/// "Point with 2 fields" for a struct, "Shape (enum)" for an enum. Already written.
fn sentence(input: &DeriveInput) -> String {
    let name = &input.ident;
    match &input.data {
        Data::Struct(s) => format!("{name} with {} fields", s.fields.len()),
        Data::Enum(_) => format!("{name} (enum)"),
        Data::Union(_) => format!("{name} (union)"),
    }
}
