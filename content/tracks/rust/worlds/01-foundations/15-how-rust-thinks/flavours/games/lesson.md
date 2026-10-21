The next quests turn your calculator into real features: recipe lookup, reading recipe lines, handling bad input. Three ideas make Rust feel different from most languages. Read them once and they will click when you meet them in code.

**The compiler is your first tester.** Types, `mut` and (later) ownership are all checked **before** the program runs. Mix a stack count with an item name and Rust refuses to build. A compile error is Rust saying "this would have been a bug". Read it: the message usually tells you the fix, like the ones you have already met.

**There is no null.** In many languages, a value can secretly be "missing" and crash you later. Rust has no such value. Looking up `"diamond_pickaxe"` in your recipe book might find nothing, so the type of the answer says so: an **Option** is either `Some(value)` or `None`, and you have to deal with `None` before you can use the value.

**There are no exceptions.** A function that can fail says so in its type, too. Reading the line `"stick x4 = plank x2"` can go wrong, so a **Result** is either `Ok(value)` or `Err(reason)`, and the caller can never forget that the failure exists.

```rust
let recipe: Option<u32> = None;             // "maybe a number"
let parsed: Result<u32, String> = Ok(4);    // "a number, or a reason it failed"
```

**Why bother.** Whole families of crashes, like missing values, uncaught errors and mixed-up types, become messages you fix in seconds instead of bugs you hunt for hours on a live server. The quests ahead teach `Option` and `Result` properly.

## Your turn

- Answer the two questions. No code this time.
