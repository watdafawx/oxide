In the last quest you passed a `&Box<Belt>` where a `&Belt` was wanted, and it just worked. A `Box` is an ordinary struct from the standard library, and what makes it act like the value inside is one trait. You will pass machines around behind handles, so build a handle of your own and see it.

**Without help.** Wrap a machine in a tuple struct, `struct InserterHandle<T>(T);`, and try `*h` on one. Rust refuses with `error[E0614]: type InserterHandle<{integer}> cannot be dereferenced`. The `*` means "the value this points at", and nothing tells Rust what a handle points at.

**The trait for it.** The `Deref` trait answers that question. `type Target` names what the handle points at, and one method, `deref`, returns a reference to it. From then on `*h` means `*h.deref()`. A type that owns a value and acts like a pointer to it this way is called a **smart pointer**.

```rust
use std::ops::Deref;

struct Chest(Vec<String>);

impl Deref for Chest {
    type Target = Vec<String>;

    fn deref(&self) -> &Vec<String> {
        &self.0
    }
}
```

**Methods come along.** With `Deref` in place, `chest.len()` works. `Chest` has no `len` of its own, so Rust tries the `Target`, a `Vec`, which does.

**The aha.** This is why a `&String` has worked wherever a `&str` was wanted since the ownership world: `String` implements `Deref` with `Target = str`. Rust follows `Deref` as many steps as it needs, so a `&InserterHandle<String>` becomes a `&String` and then a `&str`. That automatic step is called **deref coercion**.

## Your turn

- In `deref` for `InserterHandle`, return a reference to the wrapped machine, field `0`.
- Save. Both tests use your `deref`: one through `*`, one through deref coercion into `label_len`.
