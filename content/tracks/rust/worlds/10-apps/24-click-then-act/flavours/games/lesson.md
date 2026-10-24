The mod list draws one button per mod name, and clicking one selects that mod for the details panel. Drawing the buttons reads `self.names`; selecting changes `self`. Doing both at once is the most common borrow error in UI code, and the fix is a small pattern you will use in the boss.

**The obvious way.** Inside `for name in &self.names`, write `if ui.button(name.as_str()).clicked() { self.select(name); }`. It reads naturally, and it does not compile:

```text
error[E0502]: cannot borrow `*self` as mutable because it is also borrowed as immutable
   |
   |         for name in &self.names {
   |                     ----------- immutable borrow occurs here
   |             if ui.button(name.as_str()).clicked() {
   |                 self.select(name);
   |                 ^^^^^^^^^^^^^^^^^ mutable borrow occurs here
```

**Read the error.** From world 02: many readers or one writer, never both. The loop holds a reader on `self.names` until it ends, and `select(&mut self)` asks for a writer on all of `self`, names included. The compiler can't know that `select` won't change the list under the loop's feet, so it refuses.

**Decide now, act later.** Inside the loop, only remember what was clicked, in a local `Option<String>`. When the loop ends, its borrow ends too, and you are free to change `self`. A `name.clone()` gives you your own copy of the text, so it can outlive the borrow.

```rust
let mut join: Option<String> = None;
for server in &self.servers {
    if ui.button(server.as_str()).clicked() {
        join = Some(server.clone());
    }
}
if let Some(server) = join {
    self.connect(&server);
}
```

**A shape you'll see again.** Immediate-mode screens are full of it: draw while reading the state, collect what the player asked for, then apply it after the drawing. It also keeps the rule "the drawing code only reads" true, which is what made these screens easy to test.

## Your turn

- Before the loop in `ui`, make a local `Option<String>` for the clicked name, starting at `None`.
- Inside the loop, store a copy of the name that was clicked.
- After the loop, select the stored name with `self.select`.
