"How many planks from 12 logs" was fine for one chest. Now you want the answer for 7 logs, then 100, then whatever a player types. Writing `logs * 4` in a new place every time is how calculators get wrong. You want the rule written once, with a name, that works for any number.

**The word for it.** That is a **function**: a named rule. Something goes in, something comes out. You have been using two of them already; now you write one.

**Read the first line.** Read it piece by piece. `fn` starts a function. `planks` is its name. `logs: u32` is what goes in: a number called `logs`. `-> u32` is what comes out: also a number. (`u32` is Rust's name for a whole number that is never negative, which is exactly what a log count is.)

**The body.** The body sits between `{` and `}`. Its last line, written **without a semicolon**, is the answer the function hands back:

```rust
fn planks(logs: u32) -> u32 {
    logs * 4
}
```

**The one that trips people.** That no-semicolon rule is the one that trips people. A line ending in `;` is a step that does something. A line without one is a value. The function's answer is its final value.

**Use it.** Now you can **call** it: `planks(12)` is 48, `planks(7)` is 28. One rule, any number. This is the shape of every rule in the calculator from here on: name, what goes in, what comes out, one body.

**If this is new.** Programming 101 explains functions from zero: [Define and call](/quest/basics/05-functions/01-define-and-call).

## Your turn

- Fill in `planks`: four planks per log.
- Fill in `sticks`: two sticks per plank.
- Save. Press Run to see the rules used on a few chests.
