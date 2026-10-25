The task manager's list gets its real rules: sort by any column, search by name. You write those rules once, in a model with no GUI library in it, and then draw the same list in egui and in GPUI. When this is green, switching libraries later is a matter of one file.

**Three files, one list.** `model.rs` holds `TaskList`: every process, the column it is sorted by (`SortBy`), and the search text (`filter`). `egui_face.rs` and `gpui_face.rs` only draw it. Neither face sorts or searches anything itself.

**What to show.** `TaskList::visible()` decides the rows: keep the names that contain the filter, sort them, turn each into a `ProcessRow`. The CPU sort is written; Memory and Name are yours, and so is the filter.

**Biggest first.** `sort_by_key(|p| p.memory_bytes)` sorts by a key, smallest first. Wrap the key in `std::cmp::Reverse(..)` and the order flips, so 3072 MB comes before 512 MB.

**Any case.** People type `chrome` and expect `Chrome Helper.exe`. Compare lowercased copies: `to_lowercase()` on the name and on the filter, then `.contains(&needle)`. Sorting by name works the same way: the lowercased name is the key; otherwise every capital letter sorts before every small one, and `Zoom.exe` jumps to the top.

```rust
shown.sort_by_key(|p| p.pid); // smallest pid first
```

**The egui face.** The CPU button is there. Add Memory and Name the same way, and draw `list.visible()` instead of every process. Changing `list.sort` is all a button does; the next run draws the new order.

**The GPUI face.** `header(..)` builds one clickable column header that sets the sort and notifies. Add Memory and Name with it, then build one row per visible process (the same `div().flex().gap_4().child(..)` as quest 07, inside `.enumerate().map(..)` so you have `i` for the name), each named `row-0`, `row-1` and so on, and hand them to the outer box with `.children(rows)`.

**See it.** `cargo run` opens the GPUI face with five sample processes. Click the headers.

## Your turn

1. In `model.rs`, fill in the `Memory` and `Name` sorts, then the filter.
2. In `egui_face.rs`, add the Memory and Name buttons and draw `list.visible()`.
3. In `gpui_face.rs`, add the two headers and the rows.
4. Save after each step and watch the objectives turn green from the top.
