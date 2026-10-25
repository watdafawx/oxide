The `recent` command shows only the newest three chat lines, and a log dump has a header and a footer nobody wants to read. Copying those lines into a new `Vec` would cost a heap allocation per line. As with text, you can point at part of the list instead.

**A slice of a list.** `&log[1..3]` borrows items 1 and 2 of a `Vec` or an array. Its type is `&[String]` for a list of `String`s: a pointer to the first item it covers, plus a count.

**Take any list.** A parameter typed `&[String]` accepts a borrowed `Vec`, an array, or a slice of either. Rust turns a `&Vec<String>` into a `&[String]` for you, the same way it turns a `&String` into a `&str`.

```rust
fn first_two(ids: &[u32]) -> &[u32] {
    &ids[..2]
}

let ids = vec![10, 20, 30];
let head = first_two(&ids);    // [10, 20], still inside ids
```

**Check before you cut.** A range past the end panics, just like `a[i]` did in the crafting calculator, so check `.len()` first. When the answer is "nothing", give back an **empty slice**, written `&[]`.

**Count back from the end.** The last three start at `len - 3`, which would go below zero for a shorter log. `len().saturating_sub(3)` stops at 0 instead, as it did in "Stop at zero".

## Your turn

- In `trim_edges`, drop the first and the last line; give back `&[]` when there are 2 or fewer.
- In `recent_lines`, give back the last 3 lines, or all of them when there are fewer.
