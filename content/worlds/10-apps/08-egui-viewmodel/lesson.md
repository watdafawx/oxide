In egui there is no widget tree to keep in sync: your struct is the state and `ui()` draws it again every frame. That makes screens easy to test **without a window**.

- Keep logic in plain methods (`increment`, `label`). Test those like any Rust function.
- In `ui`, a widget call both draws and reports: `ui.button("+").clicked()` is `true` on the frame it was clicked.
- **egui_kittest** runs your `ui` function headlessly. It finds widgets by their text, clicks them, and steps frames.

```rust
use egui_kittest::{Harness, kittest::Queryable};

let mut harness = Harness::new_ui_state(|ui, n: &mut i32| {
    if ui.button("+").clicked() {
        *n += 1;
    }
    ui.label(format!("n = {n}"));
}, 0);

harness.get_by_label("+").click();
harness.run();                      // let egui process the click
harness.get_by_label("n = 1");      // panics if no widget says that
assert_eq!(*harness.state(), 1);
```

`get_by_label` panics when nothing matches, so a missing label fails the test with a clear message.

## Your turn

In `src/lib.rs`, on `Counter`:

- Fill in `increment`, `decrement` (stop at 0), `reset` and `label` (`"Count: 3"`).
- Fill in `ui`: a `-` button, the label, a `+` button, and a `Reset` button below.
