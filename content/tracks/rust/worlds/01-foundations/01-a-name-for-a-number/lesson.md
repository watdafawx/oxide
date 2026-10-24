Your packing planner starts with one question: how many sheets can the crates in the store room make? Before you can do any maths, the code needs a way to say "the number of crates".

**Say it in Rust.** You have 12 crates. In Rust you write that as `let crates = 12;` and read it as "let *crates* be 12". From that line on, the word `crates` means 12.

**The word for it.** A name like this is called a **variable**. You will meet that word in every error message, so keep it: a variable is a name for a value.

**Maths with names.** Once a number has a name, you can do maths with the name. One crate makes four sheets, so `crates * 4` is 48. Give that a name too:

```rust
let crates = 12;
let sheets = crates * 4;   // 48
```

**Why bother.** The point of naming is that you write the number **once**. Tomorrow the store room has 7 crates: change the `12`, and `sheets` is right everywhere, with nothing else to touch.

**What is already there.** The two functions in the starter are already written. You only fill in their `let` lines. The last line of each one, the bare name with no `;`, is how the function hands the value back. Leave it alone for now; it gets its own quest soon.

**If this is new.** Programming 101 explains variables from zero: [A name for a value](/quest/basics/02-values-and-names/02-a-name-for-a-value).

## Your turn

- In `crates_in_store`, change the `let` line so `crates` is 12.
- In `sheets_in_store`, change the `let sheets` line so it is `crates * 4`.
- Save. Both objectives turn green.
