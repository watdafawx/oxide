Every Oxide quest works the same way: a small project, a function with a gap in it, and tests that tell you when the gap is filled. This first one is a name tag for a visitor: it needs a line of text that greets them by name.

**The loop.** You edit the code in the editor. Every time you **save**, Oxide runs the tests, and the objectives above the code tick off. Save often. Nothing breaks when you do.

**The gap.** The starter already has the function written, with one line marked `todo!()`. That line is a placeholder: the project builds, but running it stops with "not done yet". Your job is to replace it.

**Text with a hole.** `format!` builds a line of text. Put `{name}` inside the quotes and Rust drops the value of `name` in there. A function that gets a visitor called `name` could build a farewell like this:

```rust
format!("See you soon, {name}!")
```

**No semicolon.** Notice the line has no `;` at the end. That is how a function hands back its answer. You only need to copy the habit for now; a later quest explains it.

**Watch it go green.** Fill in the greeting, save, and the objective turns green. Red means "not yet", never "you broke it".

**If you are new to coding.** [Programming 101](/quest/basics/01-what-a-program-is/01-instructions-in-order) teaches the ideas every language shares, once and in plain words: variables, choices, loops, functions, lists. It is optional and uses Lua; skip it if you have programmed before.

## Your turn

- In `greeting`, replace `todo!()` with a `format!` line that says `Hello, ` then the name, then `!`.
- Save. The objective turns green.
