**Why now:** you walk to the forest and pick up five more logs. The chest count has to go up. That is the first time your calculator needs a number that changes, and the first time Rust pushes back.

You already know `let logs = 10;`. The natural next line is `logs += 5;`, which reads "add 5 to logs". Try it and Rust refuses to build:

```text
error[E0384]: cannot assign twice to immutable variable `logs`
```

That word, **immutable**, means "cannot change". A plain `let` makes a name that keeps its first value for good. Rust does this on purpose: most values in a program never change, and the ones that do are where bugs like to hide, so Rust wants you to mark them.

You mark them with `mut`, short for mutable, "can change". Put it right after `let`:

```rust
let mut logs = 10;
logs += 5;        // 15
logs -= 2;        // 13: `-=` takes away
```

The habit to build: write `let` first. When you later need to change the value, the compiler tells you exactly which name needs `mut`. You do not have to guess ahead; the red message is the plan.

## Your turn

- Open `src/lib.rs`.
- In `logs_after_pickup`, add `logs += 5;` under the `let`. Save and read the error.
- Fix it the way the error says: `let mut logs`.
- `planks_after_crafting` works the same way: a crafting table costs four planks, so take 4 away.
