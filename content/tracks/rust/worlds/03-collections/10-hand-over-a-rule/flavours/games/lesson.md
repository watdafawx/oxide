Your plugin keeps asking "how many stacks are...": full, under 16, empty. Each answer is the same loop with a different rule. Write the loop once and let the caller hand you the rule as a closure.

**A closure as a parameter.** The caller writes `count_where(&stacks, |n| n == 64)`. Inside, `rule(*n)` calls whatever closure arrived. One function now answers every "how many" question.

**What type is a closure?** Every closure gets its own type, made up by the compiler, with no name you could write. So the parameter says what it must be able to do instead: `rule: impl Fn(u32) -> bool`, "something that can be called with a `u32` and gives back a `bool`".

```rust
fn enchant_twice(level: u32, upgrade: impl Fn(u32) -> u32) -> u32 {
    upgrade(upgrade(level))
}
enchant_twice(1, |l| l + 1)    // 3
```

**impl and a trait.** `Fn` is a **trait**, a skill a type can have, like Display was: here the skill "can be called like a function". `impl Fn(...)` reads "any type with that skill". This shape, `impl` plus a trait, is called **impl Trait**, and you will see it with other traits later.

**For the curious.** A function can also hand back a closure: `fn above(limit: u32) -> impl Fn(u32) -> bool { move |n| n > limit }`. The `move` makes the closure keep its own copy of `limit`, which would otherwise disappear when the function ends.

## Your turn

- In `count_where`, call `rule` on each stack and add one when it says `true`.
- In `change_all`, call `change` on each drop count and push the result.
