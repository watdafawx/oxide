Your plugin should warn the player when tools are about to break. That means walking the list of durabilities and comparing each with a limit, and the loop hands you something slightly different from a number.

**What the loop hands you.** `for d in durability` over a `&[u32]` lends you each item instead of taking it out of the list. So `d` is a `&u32`: a **reference**, an arrow pointing at one value inside the list. Write `if d < limit` and Rust stops you:

```text
error[E0308]: mismatched types
expected `&u32`, found integer
help: consider dereferencing the borrow
```

**Follow the arrow.** `*d` means "the value the arrow points at". This is called **dereferencing**. `*d < limit` compares two numbers, which is what you meant. Copying the value out works the same way: `best = *n;`.

```rust
for n in stacks {
    if *n == 64 {        // the stack size itself, not the arrow
        full += 1;
    }
}
```

**When Rust follows it for you.** Some places follow the arrow on their own. Adding works (`total += n`), and so does calling a method (`n.to_string()`). Comparing and copying into a plain `u32` do not, and the error above is how you find out.

**Why an arrow at all?** The loop lends each value so the list stays whole. Taking the items out would empty the inventory, and copying every one would be wasted work once the items are big, like whole item stacks instead of numbers.

## Your turn

- In `count_breaking`, write `if d < limit` first and read the error. Then add the `*`.
- In `biggest_stack`, keep the biggest stack seen so far in `best` and replace it when `*n` is bigger.
