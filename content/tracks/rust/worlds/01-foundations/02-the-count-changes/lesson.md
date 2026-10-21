A delivery truck arrives with five more crates. The store room count has to go up. That is the first time your planner needs a number that changes, and the first time Rust pushes back.

**Try the obvious thing.** You already know `let crates = 10;`. The natural next line is `crates += 5;`, which reads "add 5 to crates". Try it and Rust refuses to build:

```text
error[E0384]: cannot assign twice to immutable variable `crates`
```

**What the red means.** That word, **immutable**, means "cannot change". A plain `let` makes a name that keeps its first value for good. Rust does this on purpose: most values in a program never change, and the ones that do are where bugs like to hide, so Rust wants you to mark them.

**Mark it.** You mark them with `mut`, short for mutable, "can change". Put it right after `let`:

```rust
let mut crates = 10;
crates += 5;        // 15
crates -= 2;        // 13: `-=` takes away
```

**The habit.** The habit to build: write `let` first. When you later need to change the value, the compiler tells you exactly which name needs `mut`. You do not have to guess ahead; the red message is the plan.

## Your turn

- In `crates_after_delivery`, add `crates += 5;` under the `let`. Save and read the error.
- Fix it the way the error says: `let mut crates`.
- `sheets_after_packing` works the same way: packing a box uses four sheets, so take 4 away.
