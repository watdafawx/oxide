A customer asks for 5 sheets but the pile only has 3. Your planner subtracts, and a count that cannot go below zero is suddenly asked to. What happens next is the second thing Rust is strict about.

**Try the obvious thing.** `pile - amount` looks fine, and it is fine as long as the pile is big enough. Give it a pile of 3 and an amount of 5 and the program stops:

```text
thread 'main' panicked at src/lib.rs:3:5:
attempt to subtract with overflow
```

**Why it stops.** A `u32` has no room below zero. The answer would fall outside the numbers it can hold, which is called an **overflow**. A debug build treats that as a bug and panics, so you find it early.

**Subtract with a floor.** When "not enough" should simply mean "none left", use `.saturating_sub`. It subtracts like `-` does, but stops at zero instead of overflowing. The word comes from a number being "saturated" at its limit.

```rust
let left = fuel.saturating_sub(5);   // 3 fuel becomes 0, not a crash
```

**When to use it.** Any time a count must never go negative and running out is normal: a pile, a stack, a bar of health.

## Your turn

- Save and click the second objective to see the panic message.
- In `take_sheets`, replace the `-` with `.saturating_sub`.
- Save. Both objectives turn green.
