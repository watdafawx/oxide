The layers panel prints one line per shape. That function only looks at a shape, so it should borrow one, not demand a `Box` that someone has to build first.

**Borrow, don't box.** `&dyn Shape` is a borrowed trait object: a reference to some shape, its type forgotten. Unlike `Box`, it moves nothing to the heap; the shape stays where it already is.

**Any shape fits.** `describe(&rect)` and `describe(&circle)` both work. Rust turns a `&Rect` into a `&dyn Shape` for you, because `Rect` implements `Shape`.

**Boxes need one step.** A shape from your list is a `&Box<dyn Shape>`, a reference to the box, not to the shape. `m.as_ref()` borrows the shape inside the box as a `&dyn Shape`, so one function serves loose shapes and listed ones.

```rust
fn area_text(s: &dyn Shape) -> String {
    format!("{:.2}", s.area())
}

area_text(&Dot);            // a loose shape
for m in &marks {
    area_text(m.as_ref());  // a boxed one from the list
}
```

**Box or reference?** Use `Box<dyn Shape>` when something must own the shape, like the canvas list. Use `&dyn Shape` when a function only needs to look.

## Your turn

- Write `describe`: `<name> with area <area>`, with the area to one decimal place (`{:.1}`).
