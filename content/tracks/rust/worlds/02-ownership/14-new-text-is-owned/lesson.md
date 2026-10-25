`shout` makes new text inside the function and tries to hand back a borrow of it. That borrow would outlive the text it points at, and Rust catches it before the program ever runs.

**Try it.** The starter's `shout` builds `loud` and gives back `&loud`. Rust refuses:

```text
error[E0515]: cannot return reference to local variable `loud`
```

**Where the borrow points.** `loud` is owned by `shout`, so it is dropped at `shout`'s `}`. A `&str` pointing at it would point at freed memory. That is called a **dangling** reference: the use-after-free from the first quest. In C this builds fine and crashes later, somewhere else.

**Hand over the owner.** Give back the `String` itself. Returning moves it out of the function to the caller, so it is not dropped at the `}`: the caller owns it now.

```rust
fn tag(item: &str) -> String {
    let mut text = String::from("[");
    text.push_str(item);
    text.push_str("]");
    text            // moves out to the caller
}
```

**The rule of thumb.** A function may give back a borrow only of something the caller lent it. Text the function made itself goes back as a `String`.

## Your turn

- Save and read the error under `shout`.
- Change the return type to `String` and give back `loud` itself.
