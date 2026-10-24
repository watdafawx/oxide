The quest list draws one button per quest title, and clicking one opens that quest. Drawing the buttons reads `self.titles`; opening a quest changes `self`. Doing both at once is the most common borrow error in UI code, and the fix is a small pattern you will use in the boss.

**The obvious way.** Inside `for title in &self.titles`, write `if ui.button(title.as_str()).clicked() { self.open_quest(title); }`. It reads naturally, and it does not compile:

```text
error[E0502]: cannot borrow `*self` as mutable because it is also borrowed as immutable
   |
   |         for title in &self.titles {
   |                      ------------ immutable borrow occurs here
   |             if ui.button(title.as_str()).clicked() {
   |                 self.open_quest(title);
   |                 ^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
```

**Read the error.** From world 02: many readers or one writer, never both. The loop holds a reader on `self.titles` until it ends, and `open_quest(&mut self)` asks for a writer on all of `self`, titles included. The compiler can't know that `open_quest` won't change the list under the loop's feet, so it refuses.

**Decide now, act later.** Inside the loop, only remember what was clicked, in a local `Option<String>`. When the loop ends, its borrow ends too, and you are free to change `self`. A `title.clone()` gives you your own copy of the text, so it can outlive the borrow.

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

**A shape you'll see again.** Immediate-mode screens are full of it: draw while reading the state, collect what the user asked for, then apply it after the drawing. It also keeps the rule "the drawing code only reads" true, which is what made these screens easy to test.

## Your turn

- Before the loop in `ui`, make a local `Option<String>` for the clicked title, starting at `None`.
- Inside the loop, store a copy of the title that was clicked.
- After the loop, open the stored title with `self.open_quest`.
