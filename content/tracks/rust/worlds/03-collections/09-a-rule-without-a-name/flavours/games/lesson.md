A sword with Looting adds a bonus to every drop. The rule is tiny, `n + bonus`, and it needs `bonus`, which depends on the enchantment level the caller passes in. You want to write that rule once and use it for every drop.

**Try the obvious thing.** A small `fn looted(n: u32) -> u32` written inside the function looks right, but a `fn` cannot see the names around it, so `bonus` is out of reach:

```text
error[E0434]: can't capture dynamic environment in a fn item
  = help: use the `|| { ... }` closure form instead
```

**The closure form.** `|n: u32| n + bonus` is a **closure**: a small function without a name, written right where you need it. The parameters go between the bars, and the body follows. Keep it in a `let` and call it like any function: `looted(3)`.

**It sees its surroundings.** A closure can use the names around it, here `bonus`. This is called **capturing**, and it is what a `fn` cannot do. The name "closure" comes from this: it closes over the values it uses.

```rust
let fortune = 3;
let ore_drops = |n: u32| n * fortune;   // captures `fortune`
ore_drops(2)                             // 6
```

**Types are optional.** Rust can usually work out the parameter types from how you call the closure, so `|n| n * 2` is fine too. Writing `|n: u32|` while you learn makes the error messages easier to read.

## Your turn

- In `with_looting`, write a `fn` inside first and read the error. Then write the rule as a closure above the loop.
- Push `looted(*n)` for every drop. The `*` gets the number out of the reference, as before.
- `repairs_needed` is the same shape, with a rule that captures `max`.
