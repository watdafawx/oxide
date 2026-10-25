The shelf label lists what is on the shelf, `pen, mug, lamp`, and the order sheet for the supplier wants one product per line. You cut text into pieces with `split`; now you need the other direction.

**Glue with a separator.** `list.join(", ")` builds one `String` from a list of text, putting the separator between every two items. Nothing goes before the first or after the last, so there is no stray comma to trim off.

**Any separator.** The separator is any text: `", "`, `" | "`, or `"\n"`, the line break, which gives one item per line. An empty list joins to an empty `String`.

```rust
let staff = vec!["ann", "bo"];
staff.join(" & ")      // "ann & bo"
staff.join("\n")       // "ann\nbo": two lines
```

**What it joins.** `join` works on a list of text: a `Vec<String>`, a `Vec<&str>` or a slice of either. To sort first, own the list, make it `mut`, `sort` it, then `join`.

**The reverse of split.** `split(", ")` cuts at the separator and `join(", ")` puts it back, so the two undo each other. To glue with nothing in between, `concat()` is the short form of `join("")`.

## Your turn

- In `shelf_label`, join the names with `", "`.
- In `order_sheet`, sort the names, then join them with a line break.
