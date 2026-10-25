Prices on the shelf are without tax, and the till must add the tax percent to each one. The rule is tiny, `p + p * percent / 100`, and it needs `percent`, which the caller passes in. You want to write that rule once and use it for every price.

**Try the obvious thing.** A small `fn add_tax(p: u32) -> u32` written inside the function looks right, but a `fn` cannot see the names around it, so `percent` is out of reach:

```text
error[E0434]: can't capture dynamic environment in a fn item
  = help: use the `|| { ... }` closure form instead
```

**The closure form.** `|p: u32| p + p * percent / 100` is a **closure**: a small function without a name, written right where you need it. The parameters go between the bars, and the body follows. Keep it in a `let` and call it like any function: `add_tax(500)`.

**It sees its surroundings.** A closure can use the names around it, here `percent`. This is called **capturing**, and it is what a `fn` cannot do. The name "closure" comes from this: it closes over the values it uses.

```rust
let discount = 20;
let sale_price = |p: u32| p - p * discount / 100;   // captures `discount`
sale_price(500)                                       // 400
```

**Types are optional.** Rust can usually work out the parameter types from how you call the closure, so `|p| p * 2` is fine too. Writing `|p: u32|` while you learn makes the error messages easier to read.

## Your turn

- In `with_tax`, write a `fn` inside first and read the error. Then write the rule as a closure above the loop.
- Push `add_tax(*p)` for every price. The `*` gets the number out of the reference, as before.
- `top_up_amounts` is the same shape, with a rule that captures `target`.
