You get a small text index that is **correct and slow**. Your job is to make it fast without changing what it does, and to prove it with counts instead of stopwatches.

What you have, from this world:

- **A counting allocator** in `tests/index.rs` (quest 6). It tells you exactly how many allocations a call made.
- **A comparison meter**, `Word` in `src/probe.rs`. Every `==`, `cmp` and `hash` on a `Word` ticks a counter. The slow index uses it, so a linear scan costs hundreds of ticks per word.
- **The fast-path habits** (quest 8): reuse buffers, look things up by `&str`, clone only what you return.

```rust
// the shape of the fix: one lookup instead of one scan
let mut words: HashMap<String, usize> = HashMap::new();
*words.entry("rust".to_string()).or_default() += 1;
```

Rules of the fight:

- Keep `Index::build`, `search` and `top_words` exactly as declared. The tests call nothing else.
- Don't edit `tests/` or `src/probe.rs`.
- Three objectives are green on purpose: they pin the behaviour. If one turns red, your rewrite changed the answer.

Measure, change one thing, run the tests again. Go in order: the op caps first (the data structure), then the allocation caps (the buffers), then the big document.

## Your turn

In `src/lib.rs`:

- Replace the scanned `Vec` in `Index` with a `HashMap`.
- `build`: no allocation per token, no scan per token.
- `search`: one lookup.
- `top_words`: sort references and clone only the `n` results.

Press Run to play it.
