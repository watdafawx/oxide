Readers are safe because nobody changes what they look at. The moment code wants to write while a reader is still looking, Rust stops the build. Here is the rule, and the crash it saves you from.

**The rule.** At any moment a value may have **either** any number of `&` readers **or** exactly one `&mut` writer, never both at once.

**Why it matters.** `first` points at the first item's spot on the heap. `push` may find the list full, move every item to a bigger spot on the heap and free the old one. `first` would then point at freed memory: the use-after-free from the first quest. Rust refuses (the `*` in the message is the next quest's topic):

```text
error[E0502]: cannot borrow `*items` as mutable because it is also borrowed as immutable
```

**A borrow ends at its last use.** A reference only counts while it is still used further down. Finish with the reader before the writer starts, or make the reader after the writer is done, and both are fine.

```rust
let mut queue = vec![String::from("wash")];
queue.push(String::from("dry"));   // the writer is done
let first = &queue[0];              // the reader starts after
```

**Different values are fine.** Two `&mut` to two different lists never touch the same memory, so one function may take both and move items from one to the other.

## Your turn

- Save and read the error under `add_and_report`.
- Move the line that borrows the first item below the `push`.
- In `move_last`, pop from `from` and, if there was an item, push it onto `to`.
