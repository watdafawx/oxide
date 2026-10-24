Readers are safe because nobody changes what they look at. The moment a command wants to write to the player list while a reader is still looking, Rust stops the build. Here is the rule, and the server crash it saves you from.

**The rule.** At any moment a value may have **either** any number of `&` readers **or** exactly one `&mut` writer, never both at once.

**Why it matters.** `first` points at the first player's spot on the heap. `push` may find the list full, move every name to a bigger spot on the heap and free the old one. `first` would then point at freed memory: the use-after-free from the first quest. Rust refuses (the `*` in the message is the next quest's topic):

```text
error[E0502]: cannot borrow `*online` as mutable because it is also borrowed as immutable
```

**A borrow ends at its last use.** A reference only counts while it is still used further down. Finish with the reader before the writer starts, or make the reader after the writer is done, and both are fine.

```rust
let mut whitelist = vec![String::from("Steve")];
whitelist.push(String::from("Alex"));   // the writer is done
let first = &whitelist[0];               // the reader starts after
```

**Different values are fine.** Two `&mut` to two different lists never touch the same memory, so one command may take both, like moving a player from the online list to the kicked list.

## Your turn

- Save and read the error under `join_and_report`.
- Move the line that borrows the first player below the `push`.
- In `kick_last`, pop from `online` and, if there was a player, push them onto `kicked`.
