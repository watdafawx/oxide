The manifest loader will meet two kinds of trouble. A player typed `sodium@new`: that is normal life, so report it and carry on. Your own built-in table of loaders has a hole in it: that is a bug in your code. Rust has a tool for each, and picking one is a judgment call.

**Return Err for life.** A badly typed manifest line, a missing mod file, a full disk: these happen to correct launchers every day. The caller knows what to do about it: skip the line, ask again, show a message. So the function returns `Err` and lets the caller decide.

**Panic for bugs.** `panic!("...")` stops the program on the spot with a message. Use it when something happened that your own code promised could never happen, like reading slot 0 of a list you just filled. Carrying on would only spread a wrong value further.

**unwrap is a quiet panic.** `.unwrap()` on an `Err` or `None` panics with a generic message that says nothing about why. It is fine in tests and quick experiments, and a bad habit anywhere a user's input can reach it.

**expect says why.** `.expect("...")` panics too, but with your sentence. Use it on a value that truly cannot fail, and write the reason it cannot, so whoever reads the crash knows which promise broke.

```rust
let version = match text.parse::<u32>() {             // the player typed it: hand the problem back
    Ok(v) => v,
    Err(_) => return Err(ManifestError::BadVersion(text.to_string())),
};
let newest: u32 = "20".parse().expect("20 is written in the code, it always parses");
let first = built_in_loaders[0];                        // a bug if the built-in table is empty: panics
```

**The rule of thumb.** If the player can cause it, return `Err`. If only a mistake in your own code can cause it, panic. Most functions in this world return a `Result`, and the next quest shows the shortest way to pass one up.

## Your turn

- Answer the two questions. No code this time.
