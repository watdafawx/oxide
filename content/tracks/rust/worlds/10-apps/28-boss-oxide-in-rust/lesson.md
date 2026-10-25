This is the payoff: the screens of Oxide itself, rebuilt in Rust with egui. A home screen with your level, an XP bar and the next quest; a quest screen that ticks objectives when a check passes; and the Level up! pop-up. Everything is tested without a window, against a pretend engine.

**The seam is a trait.** The screens never touch the engine. They talk to `trait Api` (world 05), with three methods: `home()`, `start_quest(id)` and `run_checks(id)`. The tests hand the app a `FakeApi` that returns scripted data and writes down every call, so a test can check "Start called `start_quest` with this id". A real build would implement the same trait with Tauri commands or HTTP requests to the API from earlier in this world.

**The shapes are given.** `Home`, `Quest`, `QuestDetail`, `CheckResult`, `Completion` and `Profile` are the JSON shapes Oxide's screen really receives, trimmed, with the serde derives from quests 09 to 11. You read them; you don't change them.

**State first, drawing second.** `OxideApp` holds the state: which `screen` is showing, the open quest's `detail`, the `last` check result, a `toast` like `+50 XP`, and a pending `level_up`. Three methods change it, `start`, `run_checks` and `go_home`, and only they call the `Api`. The three `_ui` methods only read the state and report clicks, so every rule can be tested as plain Rust.

**You have every tool.** The XP bar is quest 23 (`as f32`, `clamp`, `ProgressBar`). The Start button inside `match &self.home.next` is quest 24: remember the clicked id in an `Option<String>` and call `self.start` after the match. The pop-up is quest 25. The objective lines are quest 21's `[x]`/`[ ]` text. `ui.separator()` draws a thin line between parts of a screen.

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

**One new call: keys.** `ui.input(|i| i.key_pressed(egui::Key::Escape))` is `true` on the frame Esc was pressed. `ui.input` hands a closure the frame's input state; `key_pressed` asks about one key. Ignore Esc while the level-up pop-up is open, because there it closes the pop-up instead.

**Exact texts.** The tests find widgets by text. Home: heading `Oxide`, `Level 1`, `40 / 100 XP`, the quest title as a heading, its summary, `5 min, 40 XP` and a `Start` button (or `All caught up`). Quest: the title as a heading, `0/2 objectives`, one `[ ] label` or `[x] label` per objective, the toast if any, buttons `Run checks` and `Back`. Pop-up: heading `Level up!`, `You reached level 2`, button `Continue`.

**Stage it.** The objectives go from easy to hard, and the first is a plain function. Fill in `xp_fraction` and `passed_count`, then the three state methods, then `home_ui`, `quest_ui` and `level_up_modal`, running the tests after each.

## Your turn

- Fill in every `todo!()` in `src/lib.rs`, in the order of the objectives.
- The data shapes, `Api`, `OxideApp::new` and `ui` are done for you.
- Press Run to drive the app with a fake engine and watch the state change.
