A double-xp weekend starts: every player gets a boost. When the server pauses, every name gets an AFK tag. `for x in &list` lends each item only for reading, so it can't do this.

**Lend each item to change.** `for x in &mut server.xp` walks the list and lends each item as a `&mut u32`. Whatever you do through `x` changes that item inside the list.

**Numbers need the star.** `x` is an arrow, so change the number with `*x`, as in `*x += 10`. (`*=` multiplies in place, the way `+=` adds.) Text items take methods directly: `name.push_str(" [AFK]")`.

```rust
for hunger in &mut server.food {
    *hunger = 20;
}
for name in &mut server.names {
    name.push_str(" [VIP]");
}
```

**Still one writer.** While the loop runs, the list is lent out mutably, so nothing else may read it through another name until the loop ends. Change the players inside, read the list after.

**Three ways to loop.** `for x in &list` reads each item, `for x in &mut list` changes each item, and `for x in list` with no `&` moves the whole list into the loop, which then drops it. Pick by what you need afterwards.

## Your turn

- In `boost_all`, loop over `&mut server.xp` and add 10 to each.
- In `tag_all_afk`, add `" [AFK]"` to the end of every name.
