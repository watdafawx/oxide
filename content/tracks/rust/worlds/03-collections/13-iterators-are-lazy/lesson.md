The next quests build longer chains for the stock tool: keep some products, total them, count them. One rule explains how every chain behaves, and why the compiler sometimes warns about one.

**A plan, not the work.** `prices.iter().map(|p| p * 2)` does not double anything. It builds a plan: "when someone asks, take each price and double it". Iterators are **lazy**, which means they do no work until something asks for the items.

**Asking for the items.** The step that asks is called a **consumer**. `collect()` is one: it pulls every item through the plan and gathers the answers. A `for` loop over the chain is another. The steps in between, like `map`, are called **adaptors**: each one wraps the plan in one more rule.

```rust
let plan = prices.iter().map(|p| p * 2);    // nothing has happened yet
let doubled: Vec<u32> = plan.collect();     // now every price is doubled
```

**The warning is the rule.** Write a chain with no consumer and the compiler warns `unused Map that must be used`, with the note "iterators are lazy and do nothing unless consumed". It is telling you the line does nothing.

**Why lazy?** A plan can stop early. A chain that only needs the first three answers does the work for three items, not for the whole list, and it never builds a list in between that nobody wanted.
