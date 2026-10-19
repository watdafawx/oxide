**Why now:** the boss is a chunk index that works and is slow. The temptation is to guess ("it must be the hashing") and rewrite. Programmers are bad at guessing where time goes. **Measure first.**

- **Profile a release build.** A debug build is 10 to 50 times slower and slow in different places. Use `cargo build --release` (and keep debug symbols with `[profile.release] debug = true` while profiling).
- **A sampling profiler** (samply, cargo-flamegraph, perf) checks where the program is every few milliseconds. It produces a **flamegraph**: each bar is a function, the **width** is the share of time, stacked bars are callers over callees.
- **Read it from the widest bar.** If 70% of the width is `Vec::contains` inside `build`, that is your problem, not hashing.
- **Count when time is noisy.** A stopwatch varies by machine. Allocation counts, comparison counts and call counts do not. The boss uses counts for exactly that reason.
- **Change one thing, measure again.** If the number did not move, undo it.

```text
build  ████████████████████████████░░░░░  70%   <- start here
 └ Vec::contains  █████████████████████░░
search █████░░░░░░░░░░░░░░░░░░░░░░░░░░░  15%
```

Rule of thumb: fix the algorithm (a scan that should be a lookup) before the micro-details (a clone, an extra allocation).
