use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemFn, parse_macro_input};

/// `#[counted] fn ping()` also defines `ping_calls()`: how many times `ping` has run.
#[proc_macro_attribute]
pub fn counted(_args: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let (attrs, vis, sig, block) = (&func.attrs, &func.vis, &func.sig, &func.block);
    let name = &sig.ident;
    let counter = format_ident!("{}_CALLS", name.to_string().to_uppercase());
    let reader = format_ident!("{}_calls", name);

    quote! {
        static #counter: ::std::sync::atomic::AtomicUsize = ::std::sync::atomic::AtomicUsize::new(0);

        pub fn #reader() -> usize {
            #counter.load(::std::sync::atomic::Ordering::SeqCst)
        }

        #(#attrs)*
        #vis #sig {
            #counter.fetch_add(1, ::std::sync::atomic::Ordering::SeqCst);
            #block
        }
    }
    .into()
}
