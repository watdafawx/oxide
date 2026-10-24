The boss of this world is a chunk index that gives the right answers and takes far too long to open a world. The tempting move is to guess ("it must be the lowercasing") and start rewriting. Programmers are famously bad at guessing where time goes, so the habit is: measure first, change one thing, measure again.

**Build in release.** Always time a **release build**, `cargo run --release` or `cargo build --release`. A debug build skips the compiler's optimisations; it is often 10 to 50 times slower, and slow in different places, so its numbers point you at the wrong code.

**Sample it.** A **sampling profiler** such as samply, cargo-flamegraph or perf stops the program hundreds of times a second and writes down which function it was in. Functions that show up in many samples are where the time goes, the **hot spots**.

**Read a flamegraph.** The profiler draws a **flamegraph**: each bar is a function, stacked on top of the function that called it, and its **width is its share of the time**. Start at the widest bar. If `Vec::contains` inside `build` is 70% of the width, that is your problem, whatever you guessed.

```text
build ███████████████████████████████████░░░░░░░  80%
  └ Vec::contains ████████████████████████░░░░░░  70%   <- start here
chunks_with ██████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  15%
```

**A scan inside a loop.** A **linear scan** checks a list item by item until it finds a match, so its cost grows with the list. The boss's `build` scans the list of known block names once per block it reads: 2,000 chunks of 4 blocks is 8,000 blocks, each compared with up to 300 known names, over a million comparisons. A `HashMap` finds a name in about one step, so the same build is about 8,000.

**Big-O, in one line.** Programmers write that cost as **O(n)**, said "order n": work grows in step with the input. A scan inside a loop over the same input is **O(n²)**: double the input and the work grows four times. A better data structure beats any amount of tuning.

**Fix it in order.** The algorithm first (a scan that should be a lookup), then allocations and copies, the next quests. Micro-tuning such as `unsafe`, SIMD or `#[inline]` comes last, and only when the profile says so. In the boss you measure with counts instead of seconds, so the result is the same on every machine.
