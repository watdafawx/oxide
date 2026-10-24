This is the payoff: the mod manager's screens in Rust with egui. A home screen with your library tier, a tier bar and the next modpack to install; a pack screen that ticks each mod when its file checks out; and the Tier up! pop-up. Everything is tested without a window, against a pretend engine.

**The seam is a trait.** The screens never touch the engine. They talk to `trait Api` (world 05), with three methods: `home()`, `open_pack(id)` and `verify(id)`. The tests hand the app a `FakeApi` that returns scripted data and writes down every call, so a test can check "Install called `open_pack` with this id". A real build would implement the same trait with Tauri commands or HTTP requests to the mod API from earlier in this world.

**The shapes are given.** `Home`, `Modpack`, `PackDetail`, `VerifyResult`, `Install` and `LibraryStats` are the JSON shapes the screen receives, with the serde derives from quests 09 to 11. You read them; you don't change them.

**State first, drawing second.** `ModManagerApp` holds the state: which `screen` is showing, the open pack's `detail`, the `last` verify result, a `toast` like `+50 MB`, and a pending `tier_up`. Three methods change it, `open`, `verify` and `go_home`, and only they call the `Api`. The three `_ui` methods only read the state and report clicks, so every rule can be tested as plain Rust.

**You have every tool.** The tier bar is quest 23 (`as f32`, `clamp`, `ProgressBar`). The Install button inside `match &self.home.next` is quest 24: remember the clicked id in an `Option<String>` and call `self.open` after the match. The pop-up is quest 25. The mod lines are quest 21's `[x]`/`[ ]` text. `ui.separator()` draws a thin line between parts of a screen.

```rust
let mut go: Option<String> = None;
match &self.selected {
    Some(world) => {
        ui.heading(world.name.as_str());
        if ui.button("Play").clicked() {
            go = Some(world.id.clone());
        }
    }
    None => {
        ui.label("No world selected");
    }
}
if let Some(id) = go {
    self.launch(&id);
}
```

**One new call: keys.** `ui.input(|i| i.key_pressed(egui::Key::Escape))` is `true` on the frame Esc was pressed. `ui.input` hands a closure the frame's input state; `key_pressed` asks about one key. Ignore Esc while the tier-up pop-up is open, because there it closes the pop-up instead.

**Exact texts.** The tests find widgets by text. Home: heading `Mod manager`, `Tier 1`, `40 / 100 MB`, the pack name as a heading, its summary, `fabric, 40 MB` and an `Install` button (or `Everything installed`). Pack: the name as a heading, `0/2 mods verified`, one `[ ] name` or `[x] name` per mod, the toast if any, buttons `Verify files` and `Back`. Pop-up: heading `Tier up!`, `You reached tier 2`, button `Continue`.

**Stage it.** The objectives go from easy to hard, and the first is a plain function. Fill in `mb_fraction` and `verified_count`, then the three state methods, then `home_ui`, `pack_ui` and `tier_up_modal`, running the tests after each.

## Your turn

- Fill in every `todo!()` in `src/lib.rs`, in the order of the objectives.
- The data shapes, `Api`, `ModManagerApp::new` and `ui` are done for you.
- Press Run to drive the app with a fake engine and watch the state change.
