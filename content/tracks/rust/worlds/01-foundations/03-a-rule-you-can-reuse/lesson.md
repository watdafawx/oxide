"How many sheets from 12 crates" was fine for one store room. Now you want the answer for 7 crates, then 100, then whatever a customer orders. Writing `crates * 4` in a new place every time is how planners get wrong. You want the rule written once, with a name, that works for any number.

**The word for it.** That is a **function**: a named rule. Something goes in, something comes out. You have been using two of them already; now you write one.

**Read the first line.** Read it piece by piece. `fn` starts a function. `sheets` is its name. `crates: u32` is what goes in: a number called `crates`. `-> u32` is what comes out: also a number. (`u32` is Rust's name for a whole number that is never negative, which is exactly what a crate count is.)

**The body.** The body sits between `{` and `}`. Its last line, written **without a semicolon**, is the answer the function hands back:

```rust
fn sheets(crates: u32) -> u32 {
    crates * 4
}
```

**The one that trips people.** That no-semicolon rule is the one that trips people. A line ending in `;` is a step that does something. A line without one is a value. The function's answer is its final value.

**Use it.** Now you can **call** it: `sheets(12)` is 48, `sheets(7)` is 28. One rule, any number. This is the shape of every rule in the planner from here on: name, what goes in, what comes out, one body.

**If this is new.** Programming 101 explains functions from zero: [Define and call](/quest/basics/05-functions/01-define-and-call).

## Your turn

- Fill in `sheets`: four sheets per crate.
- Fill in `boxes`: two boxes per sheet.
- Save. Press Run to see the rules used on a few store rooms.
