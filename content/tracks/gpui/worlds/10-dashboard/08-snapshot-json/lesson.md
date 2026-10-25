The web dashboard shows the same numbers in a browser, on this PC or, if you allow it, on your phone. A browser cannot read Rust structs, so the bar's numbers first have to become data a browser does read.

**What JSON is.** **JSON** is a way of writing data as plain text: a name in quotes, a colon, then the value. Curly braces hold one thing, square brackets hold a list. Every browser reads it, and so does nearly every language, which is why web APIs answer in JSON. Here is one stat as the page will get it:

```json
{"stats":[{"kind":"CpuUsage","label":"CPU","text":"30%","value":30.0,"warning":false}]}
```

**serde writes it.** `WebStat` and `WebSnapshot` already have `#[derive(Serialize)]`, the same serde you used to save `Settings`. `serde_json` writes their fields by name, in the order they are declared, and a `None` value comes out as `null`.

**Kinds as names.** serde writes each `StatKind` as its variant's name, `"CpuUsage"`, so the page can tell stats apart without a lookup table of numbers.

**Every stat.** Unlike the bar, the page shows every kind in `ALL`, read or not. Make a `Reading` per kind with its value from the snapshot, and take the label, text, value and warning from it, so the page and the bar never disagree about `13%` or `--`.

**A closure with a block.** The closure for `map` needs two steps, so its body is a `{ .. }` block: a `let` for the reading, then the `WebStat` as the last line, which is the value the block gives back. `.collect()` gathers the stats into the `Vec` the `stats` field wants.

**Warnings too.** `is_warning(&reading, &limits)` from the Readings world says whether a stat is over its limit. `Limits::default()` gives Kinetik's usual limits.

**Text and value.** The page shows `text` as it is. `value` is there for anything that wants to chart or sort, which text like `8.0 GB` is bad at.

## Your turn

- In `web_snapshot`, make `Limits::default()`.
- Map every kind in `ALL` to a `WebStat` built from its `Reading`, and collect them.
- Give back `WebSnapshot { stats }`. Save.
