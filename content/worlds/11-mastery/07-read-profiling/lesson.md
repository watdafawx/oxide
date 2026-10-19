Most slow programs are slow in one or two places, and they are rarely the ones you guess. **Measure first, then change one thing, then measure again.**

The loop:

1. **Build in release.** `cargo run --release`. Debug builds are 10-50x slower and spend time in the wrong places.
2. **Profile.** `samply record target/release/app` or `cargo flamegraph` samples where the program is, hundreds of times per second.
3. **Read the flamegraph.** Each bar is a function, stacked on its caller. **Width is time.** A wide bar near the top is a hot spot, a wide plateau is the culprit.
4. **Fix the widest bar**, re-run, compare numbers.

What to look for first:

- **Allocations**: `malloc`, `String::clone`, `Vec::push` growth. You counted these in the last quest.
- **Wrong algorithm**: a linear scan inside a loop is O(n²) no matter how tidy.
- **Needless copying**: `clone()` in a loop, `String` where `&str` works.

Micro-tuning (`unsafe`, SIMD, inlining) comes last, only after the profile says so.

The next quests practise the cheap, common fixes. In the boss you measure with *counts* instead of seconds, so your result is the same on every machine.
