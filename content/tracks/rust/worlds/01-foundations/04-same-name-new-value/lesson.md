Customers type labels by hand, and `"  Fragile "` has stray spaces that make it a different label from `"Fragile"`. Your planner has to clean the label before it uses it. Cleaning makes a new, tidier value, and you can keep the same name for it.

**Try the obvious thing.** Write `let label = "  Fragile ";` and then, on the next line, `let label = ...` again with the same name. Most languages would complain. Rust does not: there is no error at all.

**The word for it.** The second `label` **shadows** the first. From that line on, the name means the new value, and the old one is hidden for good. That trick is called **shadowing**.

**Cleaning text.** Text has a method called `.trim()`, used with a dot just like `.to_uppercase()` was. It hands back the text without the spaces at both ends. Shadow the name with the clean version:

```rust
let label = "  Handle with care ";
let label = label.trim();      // "Handle with care"
```

**Not the same as mut.** `mut` changes one value in place. Shadowing makes a brand-new value with a new `let`, and that new one still cannot change. Use `mut` for a number that keeps changing, and shadowing for "the same thing, tidied up".

**Step after step.** You can shadow as many times as you like. Every new line starts from the one before it, so two `let stock = stock + 8;` lines add 16.

## Your turn

- In `label_len`, shadow `label` with its trimmed version, then return its `.len()`, which counts the letters.
- In `sheets_after_two_boxes`, shadow `sheets` twice, taking 4 away each time.
- Save. Both objectives turn green.
