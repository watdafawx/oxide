The owner keeps asking "how many products are...": under 5 left, over 100 left, exactly 0 left. Each answer is the same loop with a different rule. Write the loop once and let the caller hand you the rule as a closure.

**A closure as a parameter.** The caller writes `count_where(&counts, |n| n < 5)`. Inside, `rule(*n)` calls whatever closure arrived. One function now answers every "how many" question.

**What type is a closure?** Every closure gets its own type, made up by the compiler, with no name you could write. So the parameter says what it must be able to do instead: `rule: impl Fn(u32) -> bool`, "something that can be called with a `u32` and gives back a `bool`".

```rust
fn apply_twice(price: u32, change: impl Fn(u32) -> u32) -> u32 {
    change(change(price))
}
apply_twice(400, |p| p / 2)    // 100
```

**impl and a trait.** `Fn` is a **trait**, a skill a type can have, like Display was: here the skill "can be called like a function". `impl Fn(...)` reads "any type with that skill". This shape, `impl` plus a trait, is called **impl Trait**, and you will see it with other traits later.

**For the curious.** A function can also hand back a closure: `fn above(limit: u32) -> impl Fn(u32) -> bool { move |n| n > limit }`. The `move` makes the closure keep its own copy of `limit`, which would otherwise disappear when the function ends.

## Your turn

- In `count_where`, call `rule` on each count and add one when it says `true`.
- In `change_all`, call `change` on each price and push the result.
