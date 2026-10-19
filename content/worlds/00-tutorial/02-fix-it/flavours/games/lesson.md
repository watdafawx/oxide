**Why now:** your name tag needs a rule for how long a player name may be. The test knows the rule; your code has it wrong. Reading the failure is how you find the fix.

Most of learning Rust is **reading what the compiler and the tests tell you**. They are precise and they are on your side.

A failing `assert_eq!` prints both sides:

```text
assertion `left == right` failed
  left: 15        <- what your code returned
 right: 16        <- what the test wanted
```

In Oxide, a red objective shows that output when you click it. Compiler errors show up above the objectives, with the file and line.

## Your turn

- Save `src/lib.rs` once without changing anything. Look at the red objective.
- Fix `max_name_len` so the test passes.
