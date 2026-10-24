The console keeps a chat log, and every entry remembers how wide it is so the log window can line them up. A chat line lives on the heap, and someone has to free it when it is no longer needed. Rust settles who that someone is before the program runs. You meet the rule today by breaking it.

**Numbers get copied.** In the crafting calculator you wrote `let total = count;` many times and both names kept working. A `u32` is four bytes on the stack, so Rust simply copies it, and nothing needs freeing.

**Text is different.** A `String` is a note pointing at letters on the heap. If `let logged = line;` copied just the note, two names would point at the same letters, and at their `}` both would free them: the double free from the first quest.

**One owner.** So Rust gives each value exactly one **owner**, the name responsible for dropping it. `let logged = line;` hands the note to `logged`. This is a **move**: `logged` owns the chat line now, and `line` may not be used again.

```rust
let line = String::from("<Steve> hi");   // the same as "<Steve> hi".to_string()
let logged = line;                       // moved: logged owns the text now
let n = line.len();                      // error: line was moved
```

**What the compiler says.** Using a name after its value moved stops the build. The message points at the line that moved it and the line that used it afterwards:

```text
error[E0382]: borrow of moved value: `line`
```

**Order matters.** Putting a `String` into a struct field moves it too. Using a value before it moves is fine, so ask for everything you need first, then hand it over.

**Lengths are usize.** `.len()` gives a **`usize`**: an unsigned whole number sized to the computer's memory. Rust uses it for every length and every index.

## Your turn

- Save and read the error under `log_line`.
- Work out the width into its own `let` before building the `ChatEntry`, then use that name for the `width` field.
