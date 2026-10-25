A pickaxe stacks to 1 while a stack of cobblestone holds 64. The calculator has to choose between two numbers depending on a fact about the item. That choice is an `if`.

**A yes or no.** A comparison like `items <= 8` gives back `true` or `false`. A value that is one or the other is a **bool**. The comparisons are `==` (equal), `!=` (not equal), `<`, `>`, `<=` and `>=`. A single `=` gives a name a value; two `==` ask a question.

**Pick one.** `if` takes a bool and runs the first block when it is true, or the `else` block when it is not. A function that gets `is_tool: bool` can answer with `if is_tool { 1 } else { 64 }`.

**An if is a value.** The block that runs is the answer, with no semicolon, exactly like a function's last line. That is why the whole `if` can sit on the last line of a function. Both blocks have to give the same type of value.

**More than two choices.** Add `else if` between them. Rust tries the conditions from the top and takes the first one that is true; `else` catches everything left.

```rust
fn sword_damage(tier: u32) -> u32 {
    if tier == 1 {
        4
    } else if tier == 2 {
        5
    } else {
        6
    }
}
```

**If this is new.** Programming 101 explains `if` and `else` from zero: [Only sometimes](/quest/basics/03-decisions/01-only-sometimes).

## Your turn

- In `stack_limit`, return 1 for a tool and 64 otherwise.
- In `coal_needed`, return 1 coal for up to 8 items, 2 coal for up to 16, and 3 for anything more.
- Save. Both objectives turn green.
