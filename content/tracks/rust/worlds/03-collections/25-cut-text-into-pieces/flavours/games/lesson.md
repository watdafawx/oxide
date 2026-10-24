A modpack ships a mod list as plain text: one mod per line, fields separated by commas, `sodium, fabric, 9000, 1.2`. Chat commands look the same: `/give Steve, diamond, 3`. Before your tool can read numbers out of them, it has to cut the text into lines and each line into fields. Text works with iterators too.

**Cut at a separator.** `line.split(',')` is an iterator over the pieces between the commas: `"Steve"`, `" diamond"`, `" 3"`. Like every iterator, it does nothing until a consumer such as `collect` asks.

**Tidy each piece.** People put spaces after commas, so each piece needs `.trim()`, which you met in the Foundations world: it cuts spaces off both ends. `.map(|a| a.trim())` does that for every piece.

```rust
let parts: Vec<&str> = "north ; east;up ".split(';').map(|p| p.trim()).collect();
// ["north", "east", "up"]
```

**Cut at line breaks.** `text.lines()` is the same idea with the line break as the separator. A file often ends with an empty line, or has blank ones in between; `filter` with `!l.is_empty()` (not empty) drops them.

**No copies.** The pieces are `&str` slices pointing into the original text, as in the ownership world, so cutting a big mod list into fields copies no letters at all.

## Your turn

- In `command_args`, split the line on commas, trim every piece and collect.
- In `mod_lines`, walk the lines, trim them, drop the empty ones and collect.
