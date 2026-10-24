In the last quest you passed a `&Box<Plan>` where a `&Plan` was wanted, and it just worked. A `Box` is an ordinary struct from the standard library, and what makes it act like the value inside is one trait. Build a small box of your own to see it.

**Without help.** Wrap a value in a tuple struct, `struct MyBox<T>(T);`, and try `*b` on one. Rust refuses with `error[E0614]: type MyBox<{integer}> cannot be dereferenced`. The `*` means "the value this points at", and nothing tells Rust what a `MyBox` points at.

**The trait for it.** The `Deref` trait answers that question. `type Target` names what the wrapper points at, and one method, `deref`, returns a reference to it. From then on `*b` means `*b.deref()`. A type that owns a value and acts like a pointer to it this way is called a **smart pointer**.

```rust
use std::ops::Deref;

struct Shelf(Vec<String>);

impl Deref for Shelf {
    type Target = Vec<String>;

    fn deref(&self) -> &Vec<String> {
        &self.0
    }
}
```

**Methods come along.** With `Deref` in place, `shelf.len()` works. `Shelf` has no `len` of its own, so Rust tries the `Target`, a `Vec`, which does.

**The aha.** This is why a `&String` has worked wherever a `&str` was wanted since the ownership world: `String` implements `Deref` with `Target = str`. Rust follows `Deref` as many steps as it needs, so a `&MyBox<String>` becomes a `&String` and then a `&str`. That automatic step is called **deref coercion**.

## Your turn

- In `deref` for `MyBox`, return a reference to the wrapped value, field `0`.
- Save. Both tests use your `deref`: one through `*`, one through deref coercion into `str_len`.
