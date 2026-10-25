use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

/// `#[counted] fn ping()` also defines `ping_calls()`: how many times `ping` has run.
#[proc_macro_attribute]
pub fn counted(_args: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let (attrs, vis, sig, block) = (&func.attrs, &func.vis, &func.sig, &func.block);
    // TODO: every tagged function gets the same two names, so two functions clash. Build a name
    // per function from `sig.ident` with format_ident!: `PING_CALLS` for the static (uppercase)
    // and `ping_calls` for the reader, and use them below instead of CALLS and calls.

    quote! {
        static CALLS: ::std::sync::atomic::AtomicUsize = ::std::sync::atomic::AtomicUsize::new(0);

        pub fn calls() -> usize {
            CALLS.load(::std::sync::atomic::Ordering::SeqCst)
        }

        #(#attrs)*
        #vis #sig {
            CALLS.fetch_add(1, ::std::sync::atomic::Ordering::SeqCst);
            #block
        }
    }
    .into()
}
