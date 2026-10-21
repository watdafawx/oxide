Your crafting calculator starts with one question: how many planks can you make from the logs in your chest? Before you can do any maths, the code needs a way to say "the number of logs".

**Say it in Rust.** You have 12 logs. In Rust you write that as `let logs = 12;` and read it as "let *logs* be 12". From that line on, the word `logs` means 12.

**The word for it.** A name like this is called a **variable**. You will meet that word in every error message, so keep it: a variable is a name for a value.

**Maths with names.** Once a number has a name, you can do maths with the name. One log makes four planks, so `logs * 4` is 48. Give that a name too:

```rust
let logs = 12;
let planks = logs * 4;   // 48
```

**Why bother.** The point of naming is that you write the number **once**. Tomorrow the chest has 7 logs: change the `12`, and `planks` is right everywhere, with nothing else to touch.

**What is already there.** The two functions in the starter are already written. You only fill in their `let` lines. The last line of each one, the bare name with no `;`, is how the function hands the value back. Leave it alone for now; it gets its own quest soon.

## Your turn

- In `logs_in_chest`, change the `let` line so `logs` is 12.
- In `planks_in_chest`, change the `let planks` line so it is `logs * 4`.
- Save. Both objectives turn green.
