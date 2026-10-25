`#[counted]` works on one function. Put it on three, `ping`, `left` and `right`, and the build stops:

```text
error[E0428]: the name `CALLS` is defined multiple times
   |     #[counted]
   |     ---------- previous definition of the value `CALLS` here
   |     #[counted]
   |     ^^^^^^^^^^ `CALLS` redefined here
```

**The same template, three times.** The macro runs once per tagged function and writes the same `static CALLS` and `fn calls()` each time. Three items with one name in one module is an error, and even if it were allowed, which function would `calls()` count?

**One name per function.** The fix is to build the names from the function's own: `ping` gets `PING_CALLS` and `ping_calls()`, `left` gets `LEFT_CALLS` and `left_calls()`. The function's name is `sig.ident`. Statics are named in capitals by convention, so the counter's name is uppercased; the compiler warns about a lowercase `static`.

**A name, not a string.** `format_ident!` from the quote crate builds an `Ident`, a name, the way `format!` builds a `String`. The difference matters in `quote!`: a `String` is pasted as a string literal, `"ping_calls"` in quotes, which cannot be a function's name; an `Ident` is pasted as the name itself.

```rust
let name = &func.sig.ident;                    // left
let reset = format_ident!("reset_{}", name);   // reset_left, a name
quote! {
    pub fn #reset() {
        // ...
    }
}
```

## Your turn

- Import `format_ident` next to `quote`.
- Build the counter's name (the function's name, uppercased, plus `_CALLS`) and the reader's name (the function's name plus `_calls`).
- In `quote!`, use them everywhere the template says `CALLS` and `calls`.
