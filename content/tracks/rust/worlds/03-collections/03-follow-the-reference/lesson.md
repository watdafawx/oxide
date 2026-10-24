The shop owner wants to know how many products are running low, so they can reorder. That means walking the stock list and comparing each count with a limit, and the loop hands you something slightly different from a number.

**What the loop hands you.** `for n in counts` over a `&[u32]` lends you each item instead of taking it out of the list. So `n` is a `&u32`: a **reference**, an arrow pointing at one count inside the list. Write `if n < limit` and Rust stops you:

```text
error[E0308]: mismatched types
expected `&u32`, found integer
help: consider dereferencing the borrow
```

**Follow the arrow.** `*n` means "the value the arrow points at". This is called **dereferencing**. `*n < limit` compares two numbers, which is what you meant. Copying the value out works the same way: `best = *n;`.

```rust
for n in counts {
    if *n > 100 {        // the count itself, not the arrow
        big += 1;
    }
}
```

**When Rust follows it for you.** Some places follow the arrow on their own. Adding works (`total += n`), and so does calling a method (`n.to_string()`). Comparing and copying into a plain `u32` do not, and the error above is how you find out.

**Why an arrow at all?** The loop lends each count so the list stays whole. Taking the items out would empty the list, and copying every one would be wasted work once the items are big, like whole products instead of numbers.

## Your turn

- In `count_low`, write `if n < limit` first and read the error. Then add the `*`.
- In `largest`, keep the biggest count seen so far in `best` and replace it when `*n` is bigger.
