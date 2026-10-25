A mob farm fills the inventory with tiny stacks: 1 rotten flesh, 2 string, 1 bone. The player wants a "trash small stacks" button. You could build a new list in a loop, but `Vec` has a method that takes a rule and does the cleaning for you.

**Keep, or throw out.** `list.retain(rule)` asks the rule about every item. Items where it says `true` stay, in their order. Items where it says `false` are removed. The list changes in place, so it must be mutable.

**The rule gets a reference.** `retain` lends each item to your closure to look at, so the closure gets a `&u32`. That is why the comparison needs a `*`: `|n| *n >= 16`.

```rust
let mut durability = vec![250, 0, 31, 0];
durability.retain(|d| *d != 0);     // [250, 31]: broken tools gone
```

**Text compares as it is.** For a `Vec<String>`, the closure gets a `&String`. Comparing it with a `&str` like `junk` works directly, `|i| i != junk`, because both are references to text.

**Why not remove in a loop?** Removing by slot while you walk the list shifts everything after it, so the next stack slides into the slot you just checked and gets skipped. `retain` does the whole job in one pass and moves each item at most once.

## Your turn

- In `trash_small_stacks`, keep only the stacks of at least `min`.
- In `drop_junk`, keep every item whose name is not `junk`.
