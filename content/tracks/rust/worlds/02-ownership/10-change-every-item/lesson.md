Every item in the basket needs the same change: double each count, tag each label as new. `for x in &list` lends each item only for reading, so it can't do this.

**Lend each item to change.** `for c in &mut basket.counts` walks the list and lends each item as a `&mut u32`. Whatever you do through `c` changes that item inside the list.

**Numbers need the star.** `c` is an arrow, so change the number with `*c`. `*=` multiplies in place, the way `+=` adds. Text items take methods directly: `label.push_str(" (new)")`.

```rust
for price in &mut shop.prices {
    *price += 1;
}
for name in &mut shop.names {
    name.push_str(" (sale)");
}
```

**Still one writer.** While the loop runs, the list is lent out mutably, so nothing else may read it through another name until the loop ends. Change the items inside, read the list after.

**Three ways to loop.** `for x in &list` reads each item, `for x in &mut list` changes each item, and `for x in list` with no `&` moves the whole list into the loop, which then drops it. Pick by what you need afterwards.

## Your turn

- In `double_all`, loop over `&mut basket.counts` and double each count.
- In `mark_all`, add `" (new)"` to the end of every label.
