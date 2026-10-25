You get a small text index that is **correct and slow**: `build` reads a text, `search` lists the lines that contain a word, `top_words` ranks the most frequent words. Your job is to make it fast without changing a single answer, and to prove it with counts instead of stopwatches, so the result is the same on every machine.

**Three green on purpose.** Run the tests first. The first three objectives already pass: they pin the behaviour. If one turns red while you work, your rewrite changed an answer, and that is the bug to fix before anything else.

**The meters.** `tests/index.rs` installs the counting allocator from quest 19, so the tests know exactly how many allocations a call made. `src/probe.rs` holds `Word`, a `String` whose every `==`, `cmp` and `hash` ticks a counter: the slow index stores its words as `Word`, so each linear scan costs hundreds of ticks.

**Rules of the fight.** Keep `Index::build`, `search` and `top_words` exactly as declared; the tests call nothing else. Don't edit `tests/` or `src/probe.rs`. Inside `Index` you may store whatever you like.

**The data structure first.** The slow part is quest 18's scan inside a loop: for each of 8,000 words, `build` walks the list of every word seen so far. A `HashMap<String, ...>` from each word to its line numbers and its count makes that one lookup, and `search` becomes one lookup too. This clears the op caps.

**Then the buffers.** In `build`, lowercase every token into one reused buffer, find a known word with `get_mut(buf.as_str())`, and copy the buffer into a key only for a new word, as in quest 24. A line that holds a word twice should list that line once: push a line number only if it is not already the last one.

**Then the ranking.** `top_words` should sort references, `(&String, count)` pairs, and clone only the `n` words it returns (quest 22). Sort with `sort_unstable_by`: the plain `sort_by` allocates a scratch buffer, `sort_unstable_by` sorts in place.

**Toolkit.** Small pieces you will want, shown on other data:

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

**Measure, change one thing.** Run the tests after every change; each failure message prints the number it measured. Go in order: the op caps (the data structure), then the allocation caps (the buffers), then the big document.

## Your turn

- Replace the scanned `Vec` in `Index` with a `HashMap`.
- `build`: no allocation per token, no scan per token.
- `search`: one lookup.
- `top_words`: sort references and clone only the `n` results.
