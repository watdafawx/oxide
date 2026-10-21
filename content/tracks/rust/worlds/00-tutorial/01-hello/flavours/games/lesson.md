Every Oxide quest works the same way: a small project, a function with a gap in it, and tests that tell you when the gap is filled. This first one is for your server: when a player joins, it needs a chat line that greets them by name.

**The loop.** You edit the code in the editor. Every time you **save**, Oxide runs the tests, and the objectives above the code tick off. Save often. Nothing breaks when you do.

**The gap.** The starter already has the function written, with one line marked `todo!()`. That line is a placeholder: the project builds, but running it stops with "not done yet". Your job is to replace it.

**Text with a hole.** `format!` builds a line of text. Put `{player}` inside the quotes and Rust drops the value of `player` in there. A function that gets a player called `player` could build a farewell like this:

```rust
format!("See you soon, {player}!")
```

**No semicolon.** Notice the line has no `;` at the end. That is how a function hands back its answer. You only need to copy the habit for now; a later quest explains it.

**Watch it go green.** Fill in the chat line, save, and the objective turns green. Red means "not yet", never "you broke it".

## Your turn

- In `join_message`, replace `todo!()` with a `format!` line that says `Welcome to the server, ` then the player, then `!`.
- Save. The objective turns green.
