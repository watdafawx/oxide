**Why now:** this is the finished chunk indexer fight: a correct but slow index you make fast, proven by counts instead of stopwatches.

You get a small index that is **correct and slow**. Your job is to make it fast without changing what it does, and to prove it with counts instead of stopwatches.

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

**Toolkit** (small pieces you will want, shown on other data):

```rust
let mut seen: HashMap<String, Vec<usize>> = HashMap::new();
if let Some(v) = seen.get_mut("rust") { v.push(3); }       // look up by &str: no String made
seen.insert(key.clone(), vec![3]);                          // copy a String only for a NEW key
let hits = seen.get("rust").cloned().unwrap_or_default();   // Option<Vec> -> Vec, empty if missing

buf.clear();                                                // reuse one String for every token
buf.extend(token.chars().flat_map(char::to_lowercase));     // lowercase into buf, no new allocation

let mut pairs: Vec<(&String, usize)> = seen.iter().map(|(w, v)| (w, v.len())).collect();
pairs.sort_unstable_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(b.0)));
// b.1.cmp(&a.1) = biggest count first; then_with breaks ties alphabetically
let top: Vec<_> = pairs.into_iter().take(3).collect();      // first 3 only
```

## Your turn

In `src/lib.rs`:

- Replace the scanned `Vec` in `Index` with a `HashMap`.
- `build`: no allocation per token, no scan per token.
- `search`: one lookup.
- `top_words`: sort references and clone only the `n` results.

Press Run to play it.
