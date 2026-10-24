The chest tooltip lists what is inside, `torch, bow, arrow`, and the modpack's `mods.txt` wants one mod per line. You cut text into pieces with `split`; now you need the other direction.

**Glue with a separator.** `list.join(", ")` builds one `String` from a list of text, putting the separator between every two items. Nothing goes before the first or after the last, so there is no stray comma to trim off.

**Any separator.** The separator is any text: `", "`, `" | "`, or `"\n"`, the line break, which gives one item per line. An empty list joins to an empty `String`.

```rust
let team = vec!["Alex", "Steve"];
team.join(" & ")      // "Alex & Steve"
team.join("\n")       // "Alex\nSteve": two lines
```

**What it joins.** `join` works on a list of text: a `Vec<String>`, a `Vec<&str>` or a slice of either. To sort first, own the list, make it `mut`, `sort` it, then `join`.

**The reverse of split.** `split(", ")` cuts at the separator and `join(", ")` puts it back, so the two undo each other. To glue with nothing in between, `concat()` is the short form of `join("")`.

## Your turn

- In `chest_summary`, join the items with `", "`.
- In `modlist_file`, sort the mod names, then join them with a line break.
