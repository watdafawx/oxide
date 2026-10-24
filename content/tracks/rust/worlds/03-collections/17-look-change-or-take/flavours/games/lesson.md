Two jobs for the loot table. A Looting sword doubles every weight, in the table itself. And the old table is being replaced: the changelog needs only its item names, and the entries will not be used again. The ownership world gave you three ways to hand over a value: lend it, lend it to change, or give it away. Walking a list has the same three.

**Look.** `table.iter()` lends each item, `&DropEntry`, as every chain so far did. It is what `for d in &table` does. You can read the items; the table stays as it was.

**Change.** `table.iter_mut()` lends each item to change, `&mut DropEntry`. It is what `for d in &mut table` does. Change a field, `d.weight += 5`, or a number with `*n += 10`, and the change lands in the table itself.

**Take.** `table.into_iter()` gives the items away, `DropEntry` itself, one by one. It is what `for d in table` does. The table is **moved** into the chain, so `table` cannot be used afterwards; in exchange you own each entry and can take its parts out without copying.

```rust
for n in durability.iter_mut() {
    *n += 10;                    // every tool in the list gets repaired a bit
}
let lines: Vec<String> = signs.into_iter().map(|s| s.text).collect();   // signs is gone now
```

**Try taking with iter.** `table.iter().map(|d| d.item)` fails with `cannot move out of d.item which is behind a shared reference`: you were lent the entry, so you cannot walk off with its name. Either `clone()` it, which copies the text, or own the table and use `into_iter()`.

## Your turn

- In `apply_looting`, walk the entries with `iter_mut` and double each `weight`.
- In `into_item_names`, try `iter()` first and read the error. Then take the entries with `into_iter()` and map each to `d.item`.
