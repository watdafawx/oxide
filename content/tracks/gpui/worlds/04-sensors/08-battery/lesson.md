Kinetik shows the laptop battery with a `+` while charging. Windows has one function for it, `GetSystemPowerStatus`, written in C. Rust can call it through the `windows` crate, but the compiler cannot check what a C function does with the memory you hand it.

**Rust's checks stop here.** Everything so far, the compiler has checked: no dangling reference, no two writers at once. A C function is outside its view. It gets an address and could write past the struct, keep the address after the call, or ignore it. Rust cannot prove any of that is fine.

**The word for it.** So the call is **unsafe**: `unsafe { .. }` marks a spot where you take over the checking, and promise the rules hold that the compiler cannot see. The `unsafe` keyword itself is taught in the Rust track's systems world; here you only need this one use.

**Say what you promised.** A `// SAFETY:` comment above the block names the promise, for the next reader. Here it is simple: `status` is a real, writable struct for the whole call. Short promises like that are why this call is easy to trust.

**Fill in a struct.** The function writes into a `SYSTEM_POWER_STATUS` you pass with `&mut`. Start from `SYSTEM_POWER_STATUS::default()`, all zeros. The call gives back a `Result`; `.ok()?` turns a failure into `None`. The same shape, asking Windows for the cursor position:

```rust
let mut cursor = POINT::default();
// SAFETY: `cursor` is a valid, writable POINT for the whole call.
unsafe { GetCursorPos(&mut cursor) }.ok()?;
```

**Decoding is given.** `decode_battery` below turns the filled-in struct into a percent and a charging flag; the next quest is about how it reads those bits.

## Your turn

- In `read_power_status`, make a default status and let `GetSystemPowerStatus` fill it in, inside `unsafe`, with a `// SAFETY:` comment.
- End with `None` when the call fails, else give back `Some(status)`.
- Save.
