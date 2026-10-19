use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemFn, parse_macro_input};

/// `#[timed] fn ping() {}` also defines `ping_calls()`, how often `ping` ran.
#[proc_macro_attribute]
pub fn timed(_args: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let (attrs, vis, sig, block) = (&func.attrs, &func.vis, &func.sig, &func.block);

    let name = &sig.ident;
    let counter = format_ident!("{}_CALLS", name.to_string().to_uppercase());
    let calls_fn = format_ident!("{}_calls", name);
    let _ = &counter;

    quote! {
        // TODO: declare `static #counter: ::std::sync::atomic::AtomicUsize = ...::new(0);`

        pub fn #calls_fn() -> usize {
            // TODO: return `#counter.load(::std::sync::atomic::Ordering::SeqCst)`
            0
        }

        #(#attrs)*
        #vis #sig {
            // TODO: `#counter.fetch_add(1, ::std::sync::atomic::Ordering::SeqCst);`
            #block
        }
    }
    .into()
}
