use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

/// `#[counted] fn add(..)` also defines `calls()`: how many times `add` has run.
#[proc_macro_attribute]
pub fn counted(_args: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let (attrs, vis, sig, block) = (&func.attrs, &func.vis, &func.sig, &func.block);
    let _ = (attrs, vis, sig, block);

    quote! {
        static CALLS: ::std::sync::atomic::AtomicUsize = ::std::sync::atomic::AtomicUsize::new(0);

        pub fn calls() -> usize {
            CALLS.load(::std::sync::atomic::Ordering::SeqCst)
        }

        // TODO: instead of the function unchanged, write it back from its parts (its attributes,
        // visibility and signature), with a body that first adds 1 to CALLS and then runs the
        // original block.
        #func
    }
    .into()
}
