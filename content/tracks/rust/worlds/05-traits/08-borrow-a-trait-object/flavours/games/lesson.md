The F3 overlay prints one line for the mob under your crosshair. That function only looks at a mob, so it should borrow one, not demand a `Box` that someone has to build first.

**Borrow, don't box.** `&dyn Entity` is a borrowed trait object: a reference to some mob, its type forgotten. Unlike `Box`, it moves nothing to the heap; the mob stays where it already is.

**Any mob fits.** `describe(&zombie)` and `describe(&creeper)` both work. Rust turns a `&Zombie` into a `&dyn Entity` for you, because `Zombie` implements `Entity`.

**Boxes need one step.** A mob from the world list is a `&Box<dyn Entity>`, a reference to the box, not to the mob. `mob.as_ref()` borrows the mob inside the box as a `&dyn Entity`, so one function serves loose mobs and listed ones.

```rust
fn hp_text(m: &dyn Entity) -> String {
    format!("{} hp", m.health())
}

hp_text(&Bat);              // a loose mob
for mob in &cave {
    hp_text(mob.as_ref());  // a boxed one from the list
}
```

**Box or reference?** Use `Box<dyn Entity>` when something must own the mob, like the world list. Use `&dyn Entity` when a function only needs to look.

## Your turn

- Write `describe`: `<name> with <health> hp`.
