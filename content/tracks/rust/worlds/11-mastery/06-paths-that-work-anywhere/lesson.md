`item!` and a new `stock_map!` work in the file that defines them. The test module here imports nothing, like the file of someone who only calls your macros, and there the starter does not compile:

```text
error[E0422]: cannot find struct, variant or union type `Item` in this scope
   |         Item { name: $name, count: $count }
   |         ^^^^ not found in this scope
help: consider importing this struct
error[E0433]: cannot find type `HashMap` in this scope
```

**Pasted at the call.** A template is pasted where the macro is called, and the names in it are looked up there. In the defining file `Item` and `HashMap` were in scope; in the caller's module they are not. The compiler's help, "consider importing", would fix this one caller, but every user of the macro would have to know which imports it needs.

**Your crate: $crate.** In a template, `$crate` means "the crate that defined this macro". `$crate::Item` finds `Item` from any module of your crate, and from other crates too, where it turns into your crate's real name.

**The standard library: ::std.** For std, write the full path, starting with two colons: `::std::collections::HashMap`. The leading `::` says "start from the list of crates, not from here", so even a caller with its own module called `std` gets the real one.

**Why macros need this and functions don't.** A function body is checked once, where it is written; only its value travels to the caller. A macro's template travels as code, so every name in it must make sense in a place you have never seen.

## Your turn

- In `item!`, name the struct through `$crate`.
- In `stock_map!`, name `HashMap` by its full path from `::std`.
