A mining trip takes 30 items out of a stack. The stack has to remember it now holds fewer. That is the first method that changes the thing it belongs to, and Rust has an opinion about that.

**Try the obvious thing.** Write a method with `&self` and put `self.count -= amount;` in it. The `&` meant "look, do not touch", so Rust refuses:

```text
error[E0594]: cannot assign to `self.count`, which is behind a `&` reference
```

**Look or change.** The fix uses a word you already know. `&self` is for looking, `&mut self` is for changing, the same `mut` you put after `let`. A method that changes its value says so in its first parameter.

**The caller says it too.** To call a `&mut self` method, the value itself has to be `mut`: `let mut chest = Chest::new("tools");`. Without it, the call fails the same way a plain `let` did in the second quest.

**Never below zero.** The count is a `u32`, so keep the old rule from earlier: `self.count.saturating_sub(amount)` stops at 0 instead of crashing.

```rust
impl Chest {
    fn add_slots(&mut self, extra: u32) {
        self.slots += extra;
    }
}

let mut chest = Chest::new("tools");
chest.add_slots(9);
```

## Your turn

- In `take`, set `self.count` to `self.count.saturating_sub(amount)`. Save and read the error.
- Fix it the way the error says: `&mut self`.
- Save. Both objectives turn green.
