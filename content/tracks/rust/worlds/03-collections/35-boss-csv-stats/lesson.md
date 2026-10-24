The shop's stock lives in a CSV file: one product per line, `name,category,price,qty`, with a header line on top. The owner wants answers from it: the money tied up per category, the biggest lines, the cheapest item in a category, and a printed table. This boss builds that tool from the pieces of this world.

**What it does.** `parse_rows` reads the whole text into a `Vec<Row>`, or stops at the first bad line and says which one. Every other function takes the rows and answers one question. The tests drive it with small CSV texts, so no file is needed.

```rust
parse_rows("name,category,price,qty\npen,office,1.50,10")
// Ok([Row { name: "pen", category: "office", price: 1.5, qty: 10 }])
parse_rows("name,category,price,qty\npen,office,abc,1")
// Err("line 2: bad price 'abc'")
```

**One line first.** `parse_line` cuts a line at the commas and trims each field. Four fields or an `Err` that says how many there were; then `map_err(...)?` on each number, as in "Say what went wrong", and the `Row` at the end.

**Then the whole file.** `lines`, `enumerate` for the line numbers, `skip(1)` for the header, `filter` for blank lines, `map` through `parse_line`, and `collect` into one `Result`, as in "All or nothing". Line numbers count from 1 over every line, the header and blank lines included.

**Totals in order.** `total_by_category` is the counter from "Count each one", with a `BTreeMap` so categories print a to z, and `+= row.total()` instead of `+= 1`.

**Biggest and cheapest.** `top_n` collects references to the rows, sorts them biggest total first with `sort_by` and `total_cmp`, since totals are `f64`, then takes `n` with `into_iter().take(n)`. `cheapest_in` filters one category and picks with `min_by`.

**The report.** A `Vec<String>` of lines: the header from `vec!`, one padded line per row pushed in a loop, then the footer, and `join("\n")` at the end. The doc comments give the exact widths.

**Take the first win.** The first objective is one line. Go top to bottom: each function uses the ones above it, and every objective you clear stays green.

## Your turn

- Do the functions in order: `Row::total`, `parse_line`, `parse_rows`, `total_by_category`, `top_n`, `cheapest_in`, `report`.
- Save after each one and watch another objective turn green.
- The doc comments above each function give the exact error and report formats.
