**Why now:** the mod manager needs a screen with "Mods enabled: 12", a `+`, a `-` and a Reset. In egui the screen is a function you run every frame. Put the **state and logic in a struct** (the view model), draw it in one `ui` method, and tests can click buttons with `egui_kittest`: no window, no GPU.

```rust
pub struct Greeter { pub name: String, pub waves: u32 }

impl Greeter {
    pub fn title(&self) -> String { format!("Hello, {}", self.name) }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label(self.title());                      // draws text
        ui.horizontal(|ui| {                         // children sit side by side
            ui.label("Waves:");
            if ui.button("Wave").clicked() { self.waves += 1; }
        });
    }
}
```

- **Immediate mode**: `ui.button("+")` draws the button **and** returns the click. No callbacks to register.
- Logic lives in methods (`increment`, `decrement`, `reset`). The `ui` method only reads state and reports clicks.
- Test the model with plain asserts. Test the screen with `Harness`: `get_by_label("+").click(); harness.run();`.
- `saturating_sub` stops a `u32` at 0 instead of panicking on underflow.

## Your turn

In `src/lib.rs`:

- `increment`, `decrement` (never below 0), `reset`, `label` (`Mods enabled: N`).
- `ui`: `-`, the label and `+` in a row, `Reset` below.
