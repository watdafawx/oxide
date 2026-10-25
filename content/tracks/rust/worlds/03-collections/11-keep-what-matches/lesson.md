Sold-out products clutter the stock list: a 0 next to a product nobody can buy. The owner wants them gone. You could build a new list in a loop, but `Vec` has a method that takes a rule and does the cleaning for you.

**Keep, or throw out.** `list.retain(rule)` asks the rule about every item. Items where it says `true` stay, in their order. Items where it says `false` are removed. The list changes in place, so it must be mutable.

**The rule gets a reference.** `retain` lends each item to your closure to look at, so the closure gets a `&u32`. That is why the comparison needs a `*`: `|n| *n > 0`.

```rust
let mut prices = vec![150, 0, 400, 0];
prices.retain(|p| *p != 0);        // [150, 400]
```

**Text compares as it is.** For a `Vec<String>`, the closure gets a `&String`. Comparing it with a `&str` like `gone` works directly, `|p| p != gone`, because both are references to text.

**Why not remove in a loop?** Removing by position while you walk the list shifts everything after it, so the next item slides into the place you just checked and gets skipped. `retain` does the whole job in one pass and moves each item at most once.

## Your turn

- In `drop_sold_out`, keep only the counts above 0.
- In `drop_discontinued`, keep every product whose name is not `gone`.
