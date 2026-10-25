A customer asks "what is the first thing on the list I can afford?" and the owner asks "is anything sold out?". Neither wants a new list: one wants a single item, the other a yes or no. Both can stop as soon as they know.

**The first one.** `.find(rule)` is a consumer that walks the chain until the rule says `true`, and hands back that item as `Some(item)`. If no item passes, you get `None`. It stops at the first match; the rest of the list is never looked at.

**Yes or no.** `.any(rule)` answers `true` as soon as one item passes, and `false` if none does. Its twin `.all(rule)` answers `true` only if every item passes, and stops at the first one that fails. On an empty list, `any` is `false` and `all` is `true`: nothing failed.

```rust
let counts = vec![12, 0, 6];
counts.iter().copied().find(|n| *n > 10)    // Some(12)
counts.iter().any(|n| *n == 0)               // true
counts.iter().all(|n| *n > 0)                // false
```

**Stars, again.** As with `filter`, `find` only lends each item to the rule, so copy first and use one `*`. `any` and `all` hand the rule the item itself, which after `.iter()` is a `&u32`: one `*` there too.

**For the curious.** `.position(rule)` is `find`'s sibling that gives back the index of the first match instead of the item.

## Your turn

- In `first_affordable`, copy the prices and find the first one at or under `max`.
- In `any_sold_out`, ask whether any count is 0.
