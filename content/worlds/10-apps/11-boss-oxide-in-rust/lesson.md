Final boss: rebuild the heart of **Oxide itself** in Rust. Home screen, quest screen, XP bar, level-up modal, all in egui, all tested without a window.

The shapes (`Home`, `Quest`, `QuestDetail`, `CheckResult`, `Completion`, `Profile`) are the ones Oxide's UI gets from its engine, trimmed to what this screen needs. They derive serde, so they round-trip with JSON like the real thing.

**The seam is a trait.** The UI never touches the engine. It talks to `trait Api`:

```rust
pub trait Api {
    fn home(&self) -> Home;
    fn start_quest(&self, id: &str) -> QuestDetail;
    fn run_checks(&self, id: &str) -> CheckOutcome;   // { result, completion }
}
```

In tests a `FakeApi` returns scripted data and records the calls. Later a real implementation can invoke Tauri commands, or a GPUI view could reuse the same trait.

**Plan the state, then draw it.** `OxideApp<A: Api>` owns the screen (`Home` or `Quest`), the current `QuestDetail`, the last `CheckResult`, an XP toast and an optional level-up `Completion`. Small methods change that state (`start`, `run_checks`, `go_home`). The `ui` methods only read it and report clicks.

Exact texts the tests look for:

| Where | Text |
|---|---|
| Home | heading `Oxide`, `Level 1`, `40 / 100 XP`, card title, button `Start` |
| Quest | heading with the title, `0/2 objectives`, `[ ] label` or `[x] label`, buttons `Run checks` and `Back` |
| Toast | `+50 XP` |
| Modal | heading `Level up!`, `You reached level 2`, button `Continue` |

Stage your work: the first objective is a pure function, then Home, then each screen in turn. Run the tests after every method.

## Your turn

In `src/lib.rs`, fill in every `todo!()` (the serde types, `Api`, `OxideApp::new` and `ui` are done for you):

- `xp_fraction`, `passed_count`
- `start`, `run_checks`, `go_home`
- `home_ui`, `quest_ui`, `level_up_modal`
