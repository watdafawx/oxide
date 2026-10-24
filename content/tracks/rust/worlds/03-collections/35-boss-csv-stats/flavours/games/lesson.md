A modpack ships with a mod list in CSV: one mod per line, `name,loader,downloads,size_mb`, with a header line on top. Everyone wants answers from it: downloads per loader, the most popular mods, the smallest mod for a loader, and a clean table to paste in the README. This boss builds that tool from the pieces of this world.

**What it does.** `parse_modlist` reads the whole text into a `Vec<Mod>`, or stops at the first bad line and says which one. Every other function takes the mods and answers one question. The tests drive it with small CSV texts, so no file is needed.

```rust
parse_modlist("name,loader,downloads,size_mb\nsodium,fabric,9000,1.2")
// Ok([Mod { name: "sodium", loader: "fabric", downloads: 9000, size_mb: 1.2 }])
parse_modlist("name,loader,downloads,size_mb\nsodium,fabric,lots,1.2")
// Err("line 2: bad downloads 'lots'")
```

**One line first.** `parse_mod` cuts a line at the commas and trims each field. Four fields or an `Err` that says how many there were; then `map_err(...)?` on each number, as in "Say what went wrong", and the `Mod` at the end.

**Then the whole file.** `lines`, `enumerate` for the line numbers, `skip(1)` for the header, `filter` for blank lines, `map` through `parse_mod`, and `collect` into one `Result`, as in "All or nothing". Line numbers count from 1 over every line, the header and blank lines included.

**Totals in order.** `downloads_by_loader` is the counter from "Count each drop", with a `BTreeMap` so loaders print a to z, and `+= m.downloads` instead of `+= 1`.

**Most and smallest.** `top_n` collects references to the mods, sorts them most downloads first with `sort_by` and `cmp`, swapped, then takes `n` with `into_iter().take(n)`. `smallest_in` filters one loader and picks with `min_by` and `total_cmp`, since sizes are `f64`.

**The report.** A `Vec<String>` of lines: the header from `vec!`, one padded line per mod pushed in a loop, then the footer, and `join("\n")` at the end. The doc comments give the exact widths.

**Take the first win.** The first objective is one line. Go top to bottom: each function uses the ones above it, and every objective you clear stays green.

## Your turn

- Do the functions in order: `Mod::size_kb`, `parse_mod`, `parse_modlist`, `downloads_by_loader`, `top_n`, `smallest_in`, `report`.
- Save after each one and watch another objective turn green.
- The doc comments above each function give the exact error and report formats.
