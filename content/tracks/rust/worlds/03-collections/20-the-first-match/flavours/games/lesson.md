Auto-craft asks "what is the first stack with more than 8 items?" and the durability warning asks "is any tool about to break?". Neither wants a new list: one wants a single item, the other a yes or no. Both can stop as soon as they know.

**The first one.** `.find(rule)` is a consumer that walks the chain until the rule says `true`, and hands back that item as `Some(item)`. If no item passes, you get `None`. It stops at the first match; the rest of the inventory is never looked at.

**Yes or no.** `.any(rule)` answers `true` as soon as one item passes, and `false` if none does. Its twin `.all(rule)` answers `true` only if every item passes, and stops at the first one that fails. On an empty list, `any` is `false` and `all` is `true`: nothing failed.

```rust
let stacks = vec![4, 64, 16];
stacks.iter().copied().find(|n| *n > 10)    // Some(64)
stacks.iter().any(|n| *n == 64)              // true
stacks.iter().all(|n| *n > 8)                // false
```

**Stars, again.** As with `filter`, `find` only lends each item to the rule, so copy first and use one `*`. `any` and `all` hand the rule the item itself, which after `.iter()` is a `&u32`: one `*` there too.

**For the curious.** `.position(rule)` is `find`'s sibling that gives back the slot of the first match instead of the item.

## Your turn

- In `first_stack_over`, copy the stack sizes and find the first one bigger than `limit`.
- In `has_fragile_tool`, ask whether any durability is 1 or less.
