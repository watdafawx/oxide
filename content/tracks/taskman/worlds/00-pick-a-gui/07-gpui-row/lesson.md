Same row, second library. `RowView` holds the `ProcessRow` you built earlier and tells GPUI how to draw it: three boxes, the name, the CPU and the memory, next to each other.

**A view.** `RowView` implements `Render`. Its `render` hands back one tree of boxes (GPUI calls them elements), and it may read `self` as it builds it. The name cell is already there.

**A box with text.** `div()` is an empty box; `.child(..)` puts something inside it, text or another box. `self.row.cpu.clone()` gives the box its own copy of the text, because the element lives on after `render` returns.

**A name for tests.** `.debug_selector(|| "cell-cpu".into())` names a box so a test can ask where it was drawn. It changes nothing on screen.

```rust
.child(div().debug_selector(|| "cell-name".into()).child(self.row.name.clone()))
```

**Side by side.** Boxes stack top to bottom unless their parent says otherwise. `.flex()` on the outer box lays its children out in a row, and `.gap_4()` puts a little space between them.

**The test.** It opens the view in a test window with no screen, then asks for each name's bounds: is it drawn, and is it right of the last one.

**The first build.** If you have not done Oxide's GPUI track, this quest builds GPUI: about 2 minutes and 2 GB, once. It needs Rust's MSVC toolchain, the Windows default from `rustup`. Cargo may warn that `proc-macro-error2` will be rejected by a future Rust: that is inside one of GPUI's dependencies, not your code.

## Your turn

1. In `render`, add a `cell-cpu` box with `self.row.cpu`, then a `cell-memory` box with `self.row.memory`.
2. Add `.flex().gap_4()` to the outer box.
3. Save.
