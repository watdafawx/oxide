The supplier's stock file is plain text: one product per line, fields separated by commas, `pen, office, 1.50, 10`. Before the stock tool can read numbers out of it, it has to cut the text into lines and each line into fields. Text works with iterators too.

**Cut at a separator.** `line.split(',')` is an iterator over the pieces between the commas: `"pen"`, `" office"`, `" 1.50"`. Like every iterator, it does nothing until a consumer such as `collect` asks.

**Tidy each piece.** People put spaces after commas, so each piece needs `.trim()`, which you met in the Foundations world: it cuts spaces off both ends. `.map(|f| f.trim())` does that for every piece.

```rust
let parts: Vec<&str> = "red ; green;blue ".split(';').map(|p| p.trim()).collect();
// ["red", "green", "blue"]
```

**Cut at line breaks.** `text.lines()` is the same idea with the line break as the separator. A file often ends with an empty line, or has blank ones in between; `filter` with `!l.is_empty()` (not empty) drops them.

**No copies.** The pieces are `&str` slices pointing into the original text, as in the ownership world, so cutting a big file into fields copies no letters at all.

## Your turn

- In `fields`, split the line on commas, trim every piece and collect.
- In `non_blank_lines`, walk the lines, trim them, drop the empty ones and collect.
