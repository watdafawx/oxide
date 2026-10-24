use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemFn, parse_macro_input};

/// `#[count_calls] fn load_chunk() {}` also defines `load_chunk_calls()`, how often `load_chunk` ran.
#[proc_macro_attribute]
pub fn count_calls(_args: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let (attrs, vis, sig, block) = (&func.attrs, &func.vis, &func.sig, &func.block);

    let name = &sig.ident;
    let counter = format_ident!("{}_CALLS", name.to_string().to_uppercase());
    let calls_fn = format_ident!("{}_calls", name);
    let _ = &counter;

    quote! {
        // TODO: declare a static named #counter: an AtomicUsize (full ::std path) starting at 0

        pub fn #calls_fn() -> usize {
            // TODO: return the counter's current value (load it with SeqCst ordering)
            0
        }

        #(#attrs)*
        #vis #sig {
            // TODO: add 1 to the counter first, then run the original body
            #block
        }
    }
    .into()
}
