`/say` broadcasts a message with a `[Server]` prefix. `announce` builds that line inside the function and tries to hand back a borrow of it. That borrow would outlive the text it points at, and Rust catches it before the server ever starts.

**Try it.** The starter's `announce` builds `line` and gives back `&line`. Rust refuses:

```text
error[E0515]: cannot return reference to local variable `line`
```

**Where the borrow points.** `line` is owned by `announce`, so it is dropped at `announce`'s `}`. A `&str` pointing at it would point at freed memory. That is called a **dangling** reference: the use-after-free from the first quest. In C this builds fine and crashes later, somewhere else.

**Hand over the owner.** Give back the `String` itself. Returning moves it out of the function to the caller, so it is not dropped at the `}`: the caller owns it now.

```rust
fn whisper(name: &str, text: &str) -> String {
    let mut msg = String::from(name);
    msg.push_str(" whispers: ");
    msg.push_str(text);
    msg             // moves out to the caller
}
```

**The rule of thumb.** A function may give back a borrow only of something the caller lent it. Text the function made itself goes back as a `String`.

## Your turn

- Save and read the error under `announce`.
- Change the return type to `String` and give back `line` itself.
